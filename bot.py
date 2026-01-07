import argparse
import os
import urllib.request
import urllib.error
import json


# Constants
GITEA_BASE_URL = 'https://gitea.mavolk.de/api/v1/repos/max/python-rq-encoding'
OPENROUTER_URL = "https://openrouter.ai/api/v1/chat/completions"
DEFAULT_MODEL = "z-ai/glm-4.7"


class BotError(Exception):
    """Custom exception for bot-related errors."""
    pass


def get_env_var(name: str) -> str:
    """
    Read and validate environment variable.

    Args:
        name: Name of the environment variable

    Returns:
        The environment variable value

    Raises:
        BotError: If the environment variable is not set
    """
    value = os.environ.get(name)
    if not value:
        raise BotError(f"Environment variable '{name}' is not set")
    return value


def get_pull_request_diff(pr_number: int, api_token: str) -> str:
    """
    Fetch the diff from a pull request.

    Args:
        pr_number: The pull request number to review
        api_token: Gitea API token for authentication

    Returns:
        The diff content as a string

    Raises:
        BotError: If fetching the diff fails
    """
    url = f'{GITEA_BASE_URL}/pulls/{pr_number}.diff'
    headers = {
        'accept': 'application/json',
        'Authorization': f'token {api_token}'
    }
    req = urllib.request.Request(url, headers=headers)

    try:
        with urllib.request.urlopen(req) as response:
            return response.read().decode('utf-8')
    except urllib.error.URLError as e:
        raise BotError(f"Failed to fetch pull request diff: {e}")


def send_to_openrouter(diff: str, api_token: str, model: str = DEFAULT_MODEL, verbose: bool = False) -> dict:
    """
    Send the diff to OpenRouter API for code review.

    Args:
        diff: The pull request diff content
        api_token: OpenRouter API token
        model: The model to use (default: z-ai/glm-4.7)
        verbose: If True, print detailed response information

    Returns:
        Parsed JSON response from OpenRouter

    Raises:
        BotError: If the API request fails
    """
    headers = {
        "Authorization": f"Bearer {api_token}",
        "Content-Type": "application/json"
    }
    data = {
        "model": model,
        "messages": [
            {
                "role": "user",
                "content": f"""
                You are a senior software engineer reviewing a code change.
                Analyze the following changes and provide a structured review:
                {diff}
                """
            }
        ]
    }
    json_data = json.dumps(data).encode('utf-8')
    req = urllib.request.Request(
        OPENROUTER_URL,
        data=json_data,
        headers=headers,
        method="POST"
    )

    try:
        with urllib.request.urlopen(req) as response:
            response_data = response.read().decode('utf-8')
            if verbose:
                print("Success! Response:")
                print(json.dumps(json.loads(response_data), indent=2))
            else:
                print("Successfully sent diff to OpenRouter for review")
            return json.loads(response_data)
    except urllib.error.HTTPError as e:
        error_body = e.read().decode('utf-8')
        raise BotError(f"HTTP Error {e.code} - {e.reason}: {error_body}")
    except urllib.error.URLError as e:
        raise BotError(f"URL Error: {e.reason}")
    except json.JSONDecodeError as e:
        raise BotError(f"Failed to parse JSON response: {e}")


def add_comment_to_issue(issue_number: int, review_text: str, cost: str, api_token: str, verbose: bool = False) -> dict:
    """
    Post the review as a comment to an issue.

    Args:
        issue_number: The issue number to comment on
        review_text: The review content from OpenRouter
        cost: The cost information from OpenRouter response
        api_token: Gitea API token for authentication
        verbose: If True, print detailed response information

    Returns:
        Parsed JSON response from Gitea

    Raises:
        BotError: If posting the comment fails
    """
    url = f'{GITEA_BASE_URL}/issues/{issue_number}/comments'
    headers = {
        'accept': 'application/json',
        'Content-Type': 'application/json',
        'Authorization': f'token {api_token}'
    }
    payload = json.dumps({"body": f"Review:\n{review_text}\nCost:{cost}"}).encode('utf-8')
    req = urllib.request.Request(
        url=url,
        data=payload,
        headers=headers,
        method='POST'
    )

    try:
        with urllib.request.urlopen(req) as response:
            response_body = response.read().decode('utf-8')
            response_status = response.status
            response_headers = dict(response.getheaders())

            if verbose:
                print(f"Status: {response_status}")
                print("Response headers:")
                for key, value in response_headers.items():
                    print(f"  {key}: {value}")
                print("\nResponse body:")
                print(response_body)

            try:
                json_response = json.loads(response_body)
                if verbose:
                    print("\nParsed JSON response:")
                    print(json.dumps(json_response, indent=2))
                else:
                    print(f"Successfully posted review comment to issue #{issue_number}")
                return json_response
            except json.JSONDecodeError:
                return {"raw_response": response_body}
    except urllib.error.HTTPError as e:
        error_body = e.read().decode('utf-8')
        raise BotError(f"HTTP Error {e.code} - {e.reason}: {error_body}")
    except urllib.error.URLError as e:
        raise BotError(f"URL/Connection Error: {e.reason}")


def handle_error(error: Exception, context: str) -> None:
    """
    Unified error handling function.

    Args:
        error: The exception that occurred
        context: Description of what operation failed
    """
    print(f"Error in {context}: {error}")
    exit(1)


def main() -> None:
    """Orchestrate the entire workflow."""
    parser = argparse.ArgumentParser(
        description="Review a pull request using OpenRouter and post the review as a comment."
    )
    parser.add_argument(
        '-p', '--pr-number',
        type=int,
        required=True,
        help='Pull request number to review'
    )
    parser.add_argument(
        '-i', '--issue-number',
        type=int,
        required=True,
        help='Issue number to comment on'
    )
    parser.add_argument(
        '-m', '--model',
        type=str,
        default=DEFAULT_MODEL,
        help=f'OpenRouter model to use (default: {DEFAULT_MODEL})'
    )
    parser.add_argument(
        '-v', '--verbose',
        action='store_true',
        help='Print detailed response information'
    )

    args = parser.parse_args()

    try:
        # Get API tokens from environment variables
        gitea_token = get_env_var('GITEA_TOKEN')
        openrouter_token = get_env_var('OPENROUTER_TOKEN')

        # Get pull request diff
        diff = get_pull_request_diff(args.pr_number, gitea_token)

        # Send to OpenRouter for review
        response = send_to_openrouter(diff, openrouter_token, args.model, args.verbose)

        # Extract review and cost
        review_text = response["choices"][0]["message"]["content"]
        cost = response["usage"]["cost"]

        # Add comment to issue
        add_comment_to_issue(args.issue_number, review_text, cost, gitea_token, args.verbose)

    except BotError as e:
        handle_error(e, "bot operation")
    except Exception as e:
        handle_error(e, "unexpected error")


if __name__ == "__main__":
    main()
