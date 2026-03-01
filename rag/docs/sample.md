# Sample Document

This is a sample markdown document for testing the RAG pipeline.

## Getting Started

Follow these steps to set up your environment:

1. Install dependencies with `pip install -r requirements.txt`
2. Copy `.env.example` to `.env` and configure your settings
3. Ensure Ollama is running with the embedding model

## Configuration Options

### Elasticsearch Settings

The following settings control Elasticsearch connectivity:

- `ELASTICSEARCH_URL`: The URL of your Elasticsearch instance
- `ELASTICSEARCH_USER`: Username for authentication
- `ELASTICSEARCH_PASSWORD`: Password for authentication
- `ELASTICSEARCH_INDEX`: Name of the index to use

### Embedding Settings

Configure the embedding model:

- `OLLAMA_BASE_URL`: URL of the Ollama API
- `OLLAMA_EMBED_MODEL`: Name of the embedding model

## Advanced Usage

### Custom Chunking

You can customize chunk size and overlap:

```python
chunker = MarkdownChunker(
    chunk_size=500,
    chunk_overlap=100
)
```

### Bulk Ingestion

For large document sets, increase the batch size:

```bash
python -m src.ingest --batch-size 100
```

## Troubleshooting

### Common Issues

1. **Connection refused**: Ensure Elasticsearch is running
2. **Authentication failed**: Check your credentials
3. **Model not found**: Pull the embedding model with `ollama pull nomic-embed-text`

### Logging

Enable debug logging for more details:

```python
import logging
logging.basicConfig(level=logging.DEBUG)
```
