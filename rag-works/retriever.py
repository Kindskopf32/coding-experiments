from __future__ import annotations

from dataclasses import dataclass

from elasticsearch import NotFoundError

from config import Config
from indexer import _build_client


@dataclass
class SearchResult:
    content: str
    source_file: str
    header_path: str
    chunk_index: int
    score: float


def retrieve(query: str, cfg: Config) -> list[SearchResult]:
    es = _build_client(cfg)

    try:
        response = es.search(
            index=cfg.es_index,
            body={
                "knn": {
                        "field": "content_embedding",
                        "k": cfg.top_k,
                        "num_candidates": 100,
                        "query_vector_builder": {
                            "text_embedding": {
                                "model_id": cfg.elser_inference_id,
                                "model_text": query
                            }
                        },
                },
                "_source": ["content", "source_file", "header_path", "chunk_index"],
            },
        )
    except NotFoundError as exc:
        error_type = getattr(exc, "error", "") or str(exc)
        if "index_not_found_exception" in error_type:
            raise RuntimeError(
                f"Index '{cfg.es_index}' not found. Run 'python main.py setup' first, "
                "then 'python main.py index <directory>'."
            ) from exc
        if "inference_not_found" in error_type or "resource_not_found" in error_type:
            raise RuntimeError(
                f"ELSER inference endpoint '{cfg.elser_inference_id}' not found. "
                "Run 'python main.py setup' and wait for the model to download."
            ) from exc
        raise

    results: list[SearchResult] = []
    for hit in response["hits"]["hits"]:
        src = hit["_source"]
        results.append(
            SearchResult(
                content=src["content"],
                source_file=src["source_file"],
                header_path=src["header_path"],
                chunk_index=src["chunk_index"],
                score=hit["_score"],
            )
        )
    return results
