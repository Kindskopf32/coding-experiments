package main

import (
	"bytes"
	"encoding/json"
	"flag"
	"fmt"
	"io"
	"net/http"
	"os"
)

// Constants
const (
	GiteaBaseURL  = "https://gitea.mavolk.de/api/v1/repos/max/misc"
	OpenRouterURL = "https://openrouter.ai/api/v1/chat/completions"
	DefaultModel  = "z-ai/glm-4.7"
)

// BotError is a custom error type for bot-related errors
type BotError struct {
	Message string
	Err     error
}

func (e *BotError) Error() string {
	if e.Err != nil {
		return fmt.Sprintf("%s: %v", e.Message, e.Err)
	}
	return e.Message
}

// NewBotError creates a new BotError
func NewBotError(message string, err error) *BotError {
	return &BotError{Message: message, Err: err}
}

// OpenRouterRequest represents the request payload for OpenRouter API
type OpenRouterRequest struct {
	Model    string    `json:"model"`
	Messages []Message `json:"messages"`
}

// Message represents a chat message
type Message struct {
	Role    string `json:"role"`
	Content string `json:"content"`
}

// OpenRouterResponse represents the response from OpenRouter API
type OpenRouterResponse struct {
	Choices []Choice `json:"choices"`
	Usage   Usage    `json:"usage"`
}

// Choice represents a choice in the OpenRouter response
type Choice struct {
	Message Message `json:"message"`
}

// Usage represents usage information in the OpenRouter response
type Usage struct {
	Cost float64 `json:"cost"`
}

// GiteaCommentRequest represents the request payload for Gitea comment API
type GiteaCommentRequest struct {
	Body string `json:"body"`
}

// GetEnvVar reads and validates an environment variable
func GetEnvVar(name string) (string, error) {
	value := os.Getenv(name)
	if value == "" {
		return "", NewBotError(fmt.Sprintf("Environment variable '%s' is not set", name), nil)
	}
	return value, nil
}

// GetPullRequestDiff fetches the diff from a pull request
func GetPullRequestDiff(prNumber int, apiToken string) (string, error) {
	url := fmt.Sprintf("%s/pulls/%d.diff", GiteaBaseURL, prNumber)

	req, err := http.NewRequest("GET", url, nil)
	if err != nil {
		return "", NewBotError("Failed to create request", err)
	}

	req.Header.Set("Accept", "application/json")
	req.Header.Set("Authorization", fmt.Sprintf("token %s", apiToken))

	client := &http.Client{}
	resp, err := client.Do(req)
	if err != nil {
		return "", NewBotError("Failed to fetch pull request diff", err)
	}
	defer resp.Body.Close()

	if resp.StatusCode != http.StatusOK {
		body, _ := io.ReadAll(resp.Body)
		return "", NewBotError(fmt.Sprintf("Failed to fetch pull request diff: HTTP %d", resp.StatusCode), fmt.Errorf("%s", body))
	}

	body, err := io.ReadAll(resp.Body)
	if err != nil {
		return "", NewBotError("Failed to read response body", err)
	}

	return string(body), nil
}

// SendToOpenRouter sends the diff to OpenRouter API for code review
func SendToOpenRouter(diff, apiToken, model string, verbose bool) (*OpenRouterResponse, error) {
	headers := map[string]string{
		"Authorization": fmt.Sprintf("Bearer %s", apiToken),
		"Content-Type":  "application/json",
	}

	payload := OpenRouterRequest{
		Model: model,
		Messages: []Message{
			{
				Role: "user",
				Content: fmt.Sprintf(`
				You are a senior software engineer reviewing a code change.
				Analyze the following changes and provide a structured review:
				%s
				`, diff),
			},
		},
	}

	jsonData, err := json.Marshal(payload)
	if err != nil {
		return nil, NewBotError("Failed to marshal JSON request", err)
	}

	req, err := http.NewRequest("POST", OpenRouterURL, bytes.NewBuffer(jsonData))
	if err != nil {
		return nil, NewBotError("Failed to create request", err)
	}

	for key, value := range headers {
		req.Header.Set(key, value)
	}

	client := &http.Client{}
	resp, err := client.Do(req)
	if err != nil {
		return nil, NewBotError("URL Error", err)
	}
	defer resp.Body.Close()

	body, err := io.ReadAll(resp.Body)
	if err != nil {
		return nil, NewBotError("Failed to read response body", err)
	}

	if resp.StatusCode != http.StatusOK {
		return nil, NewBotError(fmt.Sprintf("HTTP Error %d: %s", resp.StatusCode, string(body)), nil)
	}

	var response OpenRouterResponse
	if err := json.Unmarshal(body, &response); err != nil {
		return nil, NewBotError("Failed to parse JSON response", err)
	}

	if verbose {
		fmt.Println("Success! Response:")
		prettyJSON, _ := json.MarshalIndent(response, "", "  ")
		fmt.Println(string(prettyJSON))
	} else {
		fmt.Println("Successfully sent diff to OpenRouter for review")
	}

	return &response, nil
}

// AddCommentToIssue posts the review as a comment to an issue
func AddCommentToIssue(issueNumber int, reviewText string, cost float64, apiToken string, verbose bool) (map[string]interface{}, error) {
	url := fmt.Sprintf("%s/issues/%d/comments", GiteaBaseURL, issueNumber)

	payload := GiteaCommentRequest{
		Body: fmt.Sprintf("Review:\n%s\nCost:%f", reviewText, cost),
	}

	jsonData, err := json.Marshal(payload)
	if err != nil {
		return nil, NewBotError("Failed to marshal JSON request", err)
	}

	req, err := http.NewRequest("POST", url, bytes.NewBuffer(jsonData))
	if err != nil {
		return nil, NewBotError("Failed to create request", err)
	}

	req.Header.Set("Accept", "application/json")
	req.Header.Set("Content-Type", "application/json")
	req.Header.Set("Authorization", fmt.Sprintf("token %s", apiToken))

	client := &http.Client{}
	resp, err := client.Do(req)
	if err != nil {
		return nil, NewBotError("URL/Connection Error", err)
	}
	defer resp.Body.Close()

	body, err := io.ReadAll(resp.Body)
	if err != nil {
		return nil, NewBotError("Failed to read response body", err)
	}

	if verbose {
		fmt.Printf("Status: %d\n", resp.StatusCode)
		fmt.Println("Response headers:")
		for key, values := range resp.Header {
			for _, value := range values {
				fmt.Printf("  %s: %s\n", key, value)
			}
		}
		fmt.Println("\nResponse body:")
		fmt.Println(string(body))
	}

	if resp.StatusCode != http.StatusCreated {
		return nil, NewBotError(fmt.Sprintf("HTTP Error %d: %s", resp.StatusCode, string(body)), nil)
	}

	var result map[string]interface{}
	if err := json.Unmarshal(body, &result); err != nil {
		// Return raw response if JSON parsing fails
		return map[string]interface{}{"raw_response": string(body)}, nil
	}

	if verbose {
		fmt.Println("\nParsed JSON response:")
		prettyJSON, _ := json.MarshalIndent(result, "", "  ")
		fmt.Println(string(prettyJSON))
	} else {
		fmt.Printf("Successfully posted review comment to issue #%d\n", issueNumber)
	}

	return result, nil
}

// HandleError is a unified error handling function
func HandleError(err error, context string) {
	fmt.Printf("Error in %s: %v\n", context, err)
	os.Exit(1)
}

func main() {
	// Parse command line arguments
	prNumber := flag.Int("p", 0, "Pull request number to review (required)")
	issueNumber := flag.Int("i", 0, "Issue number to comment on (required)")
	model := flag.String("m", DefaultModel, fmt.Sprintf("OpenRouter model to use (default: %s)", DefaultModel))
	verbose := flag.Bool("v", false, "Print detailed response information")
	flag.Parse()

	// Validate required arguments
	if *prNumber == 0 {
		fmt.Println("Error: --pr-number (-p) is required")
		flag.Usage()
		os.Exit(1)
	}

	if *issueNumber == 0 {
		fmt.Println("Error: --issue-number (-i) is required")
		flag.Usage()
		os.Exit(1)
	}

	// Get API tokens from environment variables
	giteaToken, err := GetEnvVar("GITEA_TOKEN")
	if err != nil {
		HandleError(err, "getting GITEA_TOKEN")
	}

	openrouterToken, err := GetEnvVar("OPENROUTER_TOKEN")
	if err != nil {
		HandleError(err, "getting OPENROUTER_TOKEN")
	}

	// Get pull request diff
	diff, err := GetPullRequestDiff(*prNumber, giteaToken)
	if err != nil {
		HandleError(err, "fetching pull request diff")
	}

	// Send to OpenRouter for review
	response, err := SendToOpenRouter(diff, openrouterToken, *model, *verbose)
	if err != nil {
		HandleError(err, "sending to OpenRouter")
	}

	// Extract review and cost
	if len(response.Choices) == 0 {
		HandleError(fmt.Errorf("no choices in response"), "OpenRouter response")
	}

	reviewText := response.Choices[0].Message.Content
	cost := response.Usage.Cost

	// Add comment to issue
	_, err = AddCommentToIssue(*issueNumber, reviewText, cost, giteaToken, *verbose)
	if err != nil {
		HandleError(err, "posting comment to issue")
	}
}
