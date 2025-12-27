import urllib.request, urllib.error
import json


# Get diff of pull request
mr_url = 'https://gitea.mavolk.de/api/v1/repos/max/misc/pulls/3.diff'
mr_headers = {
    'accept': 'application/json',
    'Authorization': 'token redacted'
}
mr_req = urllib.request.Request(mr_url, headers=mr_headers)

try:
    with urllib.request.urlopen(mr_req) as response:
        diff = response.read().decode('utf-8')
except urllib.error.URLError as e:
    print(f"Error: {e}")
    exit(1)
    
# Use diff and LLM to do a merge request review
or_url = "https://openrouter.ai/api/v1/chat/completions"
or_headers = {
    "Authorization": "Bearer redacted",
    "Content-Type": "application/json"
}
or_data = {
    "model": "z-ai/glm-4.7",
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
or_json_data = json.dumps(or_data).encode('utf-8')
or_req = urllib.request.Request(
    or_url,
    data=or_json_data,
    headers=or_headers,
    method="POST"
)

try:
    with urllib.request.urlopen(or_req) as or_response:
        response_data = or_response.read().decode('utf-8')
        print("Success! Response:")
        print(json.dumps(json.loads(response_data), indent=2))
except urllib.error.HTTPError as e:
    print(f"HTTP Error: {e.code} - {e.reason}")
    print(e.read().decode('utf-8'))
    exit(1)
except urllib.error.URLError as e:
    print(f"URL Error: {e.reason}")
    exit(1)
except Exception as e:
    print(f"Error: {e}")
    exit(1)

# Add pull request review as a comment
comment_url = 'https://gitea.mavolk.de/api/v1/repos/max/misc/issues/2/comments'
comment_headers = {
    'accept': 'application/json',
    'Content-Type': 'application/json',
    'Authorization': 'token redacted'
}
comment_response = json.loads(response_data)
comment_payload = json.dumps({"body": f"Review:\n{comment_response["choices"][0]["message"]["content"]}\nCost:{comment_response["usage"]["cost"]}"}).encode('utf-8')
comment_req = urllib.request.Request(
    url=comment_url,
    data=comment_payload,
    headers=comment_headers,
    method='POST'
)

try:
    # Send the request and handle response
    with urllib.request.urlopen(comment_req) as response:
        # Read and decode the response
        response_body = response.read().decode('utf-8')
        response_status = response.status
        response_headers = dict(response.getheaders())
        
        print(f"Status: {response_status}")
        print("Response headers:")
        for key, value in response_headers.items():
            print(f"  {key}: {value}")
        print("\nResponse body:")
        print(response_body)
        
        # If response is JSON, you can parse it
        try:
            json_response = json.loads(response_body)
            print("\nParsed JSON response:")
            print(json.dumps(json_response, indent=2))
        except json.JSONDecodeError:
            print("Response is not valid JSON")

except urllib.error.HTTPError as e:
    # Handle HTTP errors (4xx, 5xx)
    print(f"HTTP Error: {e.code} - {e.reason}")
    print(f"Response body: {e.read().decode('utf-8')}")
except urllib.error.URLError as e:
    # Handle URL/connection errors
    print(f"URL/Connection Error: {e.reason}")
except Exception as e:
    # Handle any other exceptions
    print(f"Unexpected error: {e}")
