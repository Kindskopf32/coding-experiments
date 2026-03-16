import requests
import base64
import json
from typing import Optional

class ImageToTextClient:
    def __init__(self, api_key: str, base_url: str):
        """
        Initialize the client with API key and endpoint URL.

        Args:
            api_key (str): Your API key for authentication.
            base_url (str): Base URL of the OpenAI-compatible endpoint.
        """
        self.api_key = api_key
        self.base_url = base_url.rstrip('/')
        self.headers = {
            "Authorization": f"Bearer {api_key}",
            "Content-Type": "application/json"
        }

    def encode_image_to_base64(self, image_path: str) -> str:
        """Read and encode a local image file to base64."""
        with open(image_path, "rb") as img_file:
            return base64.b64encode(img_file.read()).decode('utf-8')

    def image_to_base64(self, image_url: str) -> str:
        """Fetch and encode an image from a URL to base64."""
        response = requests.get(image_url)
        if response.status_code != 200:
            raise Exception(f"Failed to fetch image: {response.status_code}")
        return base64.b64encode(response.content).decode('utf-8')

    def send_image_request(
        self,
        image_source: str,
        prompt: str,
        is_url: bool = False,
        model: str = "gpt-4-vision-preview",
        max_tokens: int = 6000
    ) -> dict:
        """
        Send an image to the model and receive a text response.

        Args:
            image_source (str): Local file path or image URL.
            prompt (str): The text prompt to send along with the image.
            is_url (bool): Whether the image_source is a URL.
            model (str): Model name to use.
            max_tokens (int): Maximum number of tokens in the response.

        Returns:
            dict: The response JSON from the API.
        """
        if is_url:
            base64_image = self.image_to_base64(image_source)
            image_url = f"data:image/jpeg;base64,{base64_image}"
        else:
            base64_image = self.encode_image_to_base64(image_source)
            image_url = f"data:image/jpeg;base64,{base64_image}"

        payload = {
            "model": model,
            "messages": [
                {
                    "role": "user",
                    "content": [
                        {"type": "text", "text": prompt},
                        {"type": "image_url", "image_url": {"url": image_url, "detail": "auto"}}
                    ]
                }
            ],
            "max_tokens": max_tokens
        }

        try:
            response = requests.post(
                f"{self.base_url}/chat/completions",
                headers=self.headers,
                json=payload,
                timeout=600
            )
            response.raise_for_status()
            return response.json()
        except requests.exceptions.RequestException as e:
            print(f"Request failed: {e}")
            raise

def main():
    # Setup configuration
    API_KEY = "Nothing"
    BASE_URL = "http://bazzite-nvidia.host.mavolk.de:8080/v1"  # Replace with your endpoint
    MODEL = "unsloth/Qwen3.5-35B-A3B-GGUF"

    client = ImageToTextClient(api_key=API_KEY, base_url=BASE_URL)

    # Choose image source and prompt
    #image_path = "path/to/your/image.jpg"  # or use an image URL
    image_path = "/home/max/Pictures/bad-qr-code.png"
    prompt = "What is in this image? Can you decode this QR code?"

    try:
        response = client.send_image_request(
            image_source=image_path,
            prompt=prompt,
            is_url=False,
            model=MODEL
        )

        # Extract and print the response
        content = response.get("choices", [{}])[0].get("message", {}).get("content", "")
        print("Response:")
        print(content)

    except Exception as e:
        print(f"Error: {e}")

if __name__ == "__main__":
    main()
