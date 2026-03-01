#!/usr/bin/env python3
from pathlib import Path
from typing import Optional
from tqdm import tqdm

from .config import config
from .chunker import MarkdownChunker
from .embeddings import EmbeddingClient
from .elasticsearch_client import ElasticsearchClient


def run_ingestion(
    docs_path: Optional[str] = None,
    recreate_index: bool = False,
    batch_size: int = 50
) -> dict:
    docs_dir = Path(docs_path or config.docs_path)
    
    chunker = MarkdownChunker(
        chunk_size=config.chunk_size,
        chunk_overlap=config.chunk_overlap
    )
    
    embedding_client = EmbeddingClient()
    es_client = ElasticsearchClient()
    
    if not es_client.test_connection():
        raise ConnectionError("Failed to connect to Elasticsearch")
    
    if recreate_index:
        es_client.delete_index()
        print(f"Deleted existing index: {es_client.index_name}")
    
    created = es_client.create_index()
    if created:
        print(f"Created index: {es_client.index_name}")
    else:
        print(f"Using existing index: {es_client.index_name}")
    
    all_documents = []
    file_count = 0
    chunk_count = 0
    
    md_files = list(docs_dir.rglob("*.md"))
    if not md_files:
        print(f"No markdown files found in {docs_dir}")
        return {"files": 0, "chunks": 0, "indexed": 0}
    
    for md_file in tqdm(md_files, desc="Processing files"):
        chunks = chunker.chunk_file(md_file)
        if not chunks:
            continue
        
        documents = embedding_client.embed_chunks(chunks, show_progress=False)
        all_documents.extend(documents)
        file_count += 1
        chunk_count += len(chunks)
        
        if len(all_documents) >= batch_size:
            indexed = es_client.index_documents(all_documents)
            print(f"Indexed {indexed} documents")
            all_documents = []
    
    if all_documents:
        indexed = es_client.index_documents(all_documents)
        print(f"Indexed {indexed} documents")
    
    total_indexed = es_client.count_documents()
    
    return {
        "files": file_count,
        "chunks": chunk_count,
        "indexed": total_indexed
    }


def main():
    import argparse
    
    parser = argparse.ArgumentParser(description="Ingest markdown files into Elasticsearch for RAG")
    parser.add_argument("--docs-path", type=str, help="Path to documents directory")
    parser.add_argument("--recreate-index", action="store_true", help="Delete and recreate the index")
    parser.add_argument("--batch-size", type=int, default=50, help="Batch size for indexing")
    
    args = parser.parse_args()
    
    print("Starting RAG ingestion pipeline...")
    print(f"Documents path: {args.docs_path or config.docs_path}")
    print(f"Embedding model: {config.ollama_embed_model}")
    print(f"Elasticsearch index: {config.elasticsearch_index}")
    print("-" * 50)
    
    stats = run_ingestion(
        docs_path=args.docs_path,
        recreate_index=args.recreate_index,
        batch_size=args.batch_size
    )
    
    print("-" * 50)
    print("Ingestion complete!")
    print(f"Files processed: {stats['files']}")
    print(f"Chunks created: {stats['chunks']}")
    print(f"Total documents indexed: {stats['indexed']}")


if __name__ == "__main__":
    main()
