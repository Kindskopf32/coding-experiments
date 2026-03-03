from __future__ import annotations

import os
from dataclasses import dataclass
from pathlib import Path

from dotenv import load_dotenv

load_dotenv()


@dataclass(frozen=True)
class Config:
    es_url: str
    es_user: str | None
    es_password: str | None
    es_index: str
    es_pipeline: str
    elser_inference_id: str
    openrouter_api_key: str
    openrouter_model: str
    openrouter_base_url: str
    top_k: int


def load_config() -> Config:
    missing: list[str] = []

    openrouter_api_key = os.getenv("OPENROUTER_API_KEY", "")
    if not openrouter_api_key:
        missing.append("OPENROUTER_API_KEY")

    if missing:
        raise ValueError(
            f"Missing required environment variable(s): {', '.join(missing)}\n"
            "Copy .env.example to .env and fill in the required values."
        )

    return Config(
        es_url=os.getenv("ES_URL", "http://localhost:9200"),
        es_user=os.getenv("ES_USER") or None,
        es_password=os.getenv("ES_PASSWORD") or None,
        es_index=os.getenv("ES_INDEX", "rag_docs"),
        es_pipeline=os.getenv("ES_PIPELINE", "rag_elser_pipeline"),
        elser_inference_id=os.getenv("ELSER_INFERENCE_ID", "elser-2"),
        openrouter_api_key=openrouter_api_key,
        openrouter_model=os.getenv("OPENROUTER_MODEL", "anthropic/claude-sonnet-4-6"),
        openrouter_base_url=os.getenv("OPENROUTER_BASE_URL", "https://openrouter.ai/api/v1"),
        top_k=int(os.getenv("TOP_K", "5")),
    )
