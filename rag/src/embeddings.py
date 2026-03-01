from typing import Optional
import ollama
from tqdm import tqdm

from .config import config


class EmbeddingClient:
    def __init__(
        self,
        model: Optional[str] = None,
        base_url: Optional[str] = None
    ):
        self.model = model or config.ollama_embed_model
        self.client = ollama.Client(host=base_url or config.ollama_base_url)
    
    def embed(self, text: str) -> list[float]:
        response = self.client.embeddings(
            model=self.model,
            prompt=text
        )
        return response["embedding"]
    
    def embed_batch(self, texts: list[str], show_progress: bool = True) -> list[list[float]]:
        embeddings = []
        iterator = tqdm(texts, desc="Generating embeddings") if show_progress else texts
        
        for text in iterator:
            embedding = self.embed(text)
            embeddings.append(embedding)
        
        return embeddings
    
    def embed_chunks(self, chunks: list, show_progress: bool = True) -> list[dict]:
        texts = [chunk.content for chunk in chunks]
        embeddings = self.embed_batch(texts, show_progress)
        
        documents = []
        for chunk, embedding in zip(chunks, embeddings):
            documents.append({
                "content": chunk.content,
                "embedding": embedding,
                "metadata": chunk.metadata
            })
        
        return documents
