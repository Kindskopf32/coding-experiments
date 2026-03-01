# Elasticsearch Configuration for RAG Pipeline

## Overview

This guide covers configuring Elasticsearch 8.x for vector similarity search with the nomic-embed-text embedding model (768 dimensions).

## Prerequisites

- Elasticsearch 8.x running and accessible
- `nomic-embed-text` model pulled in Ollama (`ollama pull nomic-embed-text`)

## Index Mapping

Create an index with a `dense_vector` field for embeddings:

```json
PUT /rag_documents
{
  "mappings": {
    "properties": {
      "content": {
        "type": "text"
      },
      "embedding": {
        "type": "dense_vector",
        "dims": 768,
        "index": true,
        "similarity": "cosine"
      },
      "metadata": {
        "properties": {
          "source": {
            "type": "keyword"
          },
          "header_1": {
            "type": "keyword"
          },
          "header_2": {
            "type": "keyword"
          },
          "header_3": {
            "type": "keyword"
          }
        }
      },
      "created_at": {
        "type": "date"
      }
    }
  }
}
```

### Field Explanations

| Field | Type | Purpose |
|-------|------|---------|
| `content` | text | Original chunk text for retrieval |
| `embedding` | dense_vector | 768-dim vector from nomic-embed-text |
| `metadata.source` | keyword | Source file path for filtering |
| `metadata.header_*` | keyword | Markdown header hierarchy for context |
| `created_at` | date | Timestamp for tracking |

## KNN Search Configuration

Elasticsearch 8.x uses approximate kNN search via the `_search` endpoint:

```json
POST /rag_documents/_search
{
  "knn": {
    "field": "embedding",
    "query_vector_builder": {
      "text_embedding": {
        "model_id": "your_model_id",
        "model_text": "search query"
      }
    },
    "k": 10,
    "num_candidates": 100
  }
}
```

For external embeddings (Ollama), use the standard kNN query:

```json
POST /rag_documents/_search
{
  "knn": {
    "field": "embedding",
    "query_vector": [0.1, 0.2, ...],
    "k": 10,
    "num_candidates": 100
  }
}
```

## Authentication Setup

### Option 1: Basic Auth (Username/Password)

```bash
# Set password for elastic user
./bin/elasticsearch-reset-password -u elastic

# Use in connection
ELASTICSEARCH_URL=http://localhost:9200
ELASTICSEARCH_USER=elastic
ELASTICSEARCH_PASSWORD=your_password
```

### Option 2: API Key

```bash
# Create API key
curl -u elastic:password -X POST "http://localhost:9200/_security/api_key" -H "Content-Type: application/json" -d '{"name": "rag-pipeline"}'

# Response includes encoded API key
# Use as: Authorization: ApiKey <encoded_key>
```

### Option 3: Disable Security (Development Only)

In `elasticsearch.yml`:

```yaml
xpack.security.enabled: false
```

## Connection Verification

Test your connection:

```bash
# Check cluster health
curl -u elastic:password http://localhost:9200/_cluster/health

# Check index exists
curl -u elastic:password http://localhost:9200/rag_documents

# Check document count
curl -u elastic:password http://localhost:9200/rag_documents/_count
```

## Performance Tuning

### Index Settings

```json
PUT /rag_documents/_settings
{
  "index": {
    "knn": true,
    "knn.algo_param.ef_search": 100
  }
}
```

### Bulk Indexing

For large datasets, use bulk API:

```json
POST /_bulk
{"index": {"_index": "rag_documents"}}
{"content": "...", "embedding": [...], "metadata": {...}}
{"index": {"_index": "rag_documents"}}
{"content": "...", "embedding": [...], "metadata": {...}}
```

### Refresh Strategy

Disable auto-refresh during bulk ingestion:

```json
PUT /rag_documents/_settings
{
  "index": {
    "refresh_interval": "-1"
  }
}
```

Re-enable after ingestion:

```json
PUT /rag_documents/_settings
{
  "index": {
    "refresh_interval": "1s"
  }
}
```

## Common Issues

### 1. Dimension Mismatch

Error: `vector dimension mismatch`

Solution: Ensure embedding dimensions match mapping (768 for nomic-embed-text)

### 2. Connection Refused

Error: `Connection refused`

Solution: Verify Elasticsearch is running and URL is correct

### 3. Authentication Failed

Error: `401 Unauthorized`

Solution: Check credentials in `.env` file

## Docker Compose (Reference)

If you need a local Elasticsearch instance:

```yaml
version: '3.8'
services:
  elasticsearch:
    image: docker.elastic.co/elasticsearch/elasticsearch:8.11.0
    environment:
      - discovery.type=single-node
      - xpack.security.enabled=false
      - "ES_JAVA_OPTS=-Xms512m -Xmx512m"
    ports:
      - "9200:9200"
    volumes:
      - es_data:/usr/share/elasticsearch/data

volumes:
  es_data:
```

Start with: `docker-compose up -d`
