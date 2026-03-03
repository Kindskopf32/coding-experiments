from __future__ import annotations

import sys
from pathlib import Path

from elasticsearch import Elasticsearch, ConflictError
from elasticsearch.helpers import bulk

from config import Config
from chunker import chunk_file


def _build_client(cfg: Config) -> Elasticsearch:
    kwargs: dict = {"hosts": [cfg.es_url]}
    if cfg.es_user and cfg.es_password:
        kwargs["basic_auth"] = (cfg.es_user, cfg.es_password)
    return Elasticsearch(**kwargs, request_timeout=600)


def setup(cfg: Config) -> None:
    es = _build_client(cfg)

    # 2. Create ingest pipeline (PUT is a true upsert — always safe)
    es.ingest.put_pipeline(
        id=cfg.es_pipeline,
        body={
            "description": "ELSER sparse embedding pipeline",
            "processors": [
                {
                    "inference": {
                        "model_id": cfg.elser_inference_id,
                        "input_output": [
                            {"input_field": "content", "output_field": "content_embedding"}
                        ],
                    }
                }
            ],
        },
    )
    print(f"[setup] Ingest pipeline '{cfg.es_pipeline}' created/updated.")

    # 3. Create index (skip if already exists)
    if not es.indices.exists(index=cfg.es_index):
        es.indices.create(
            index=cfg.es_index,
            body={
                "settings": {"default_pipeline": cfg.es_pipeline},
                "mappings": {
                    "properties": {
                        "content": {"type": "text"},
                        "content_embedding": {"type": "dense_vector"},
                        "source_file": {"type": "keyword"},
                        "header_path": {"type": "keyword"},
                        "chunk_index": {"type": "integer"},
                    }
                },
            },
        )
        print(f"[setup] Index '{cfg.es_index}' created.")
    else:
        print(f"[setup] Index '{cfg.es_index}' already exists.")


def index_directory(directory: Path, cfg: Config) -> None:
    es = _build_client(cfg)
    md_files = sorted(directory.rglob("*.md"))

    if not md_files:
        print(f"[index] No .md files found in {directory}")
        return

    actions = []
    total_chunks = 0

    for md_file in md_files:
        chunks = chunk_file(md_file)
        print(f"[index] {md_file.name}: {len(chunks)} chunk(s)")
        for chunk in chunks:
            actions.append(
                {
                    "_index": cfg.es_index,
                    "_source": {
                        "content": chunk.content,
                        "source_file": chunk.source_file,
                        "header_path": chunk.header_path,
                        "chunk_index": chunk.chunk_index,
                        # content_embedding is populated by the ingest pipeline
                    },
                }
            )
        total_chunks += len(chunks)

    print(f"[index] Indexing {total_chunks} chunks from {len(md_files)} file(s)...")

    success_count, errors = bulk(es, actions, raise_on_error=False)
    print(f"[index] Indexed {success_count} chunks successfully.")

    if errors:
        print(f"[index] {len(errors)} error(s) during bulk indexing:", file=sys.stderr)
        for err in errors[:10]:
            print(f"  {err}", file=sys.stderr)
        if len(errors) > 10:
            print(f"  ... and {len(errors) - 10} more.", file=sys.stderr)
        sys.exit(1)
