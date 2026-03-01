from datetime import datetime
from typing import Optional
from elasticsearch import Elasticsearch
from elasticsearch.helpers import bulk

from .config import config


class ElasticsearchClient:
    def __init__(
        self,
        index_name: Optional[str] = None,
        url: Optional[str] = None,
        user: Optional[str] = None,
        password: Optional[str] = None
    ):
        self.index_name = index_name or config.elasticsearch_index
        self.es = Elasticsearch(
            [url or config.elasticsearch_url],
            basic_auth=(user or config.elasticsearch_user, password or config.elasticsearch_password),
            verify_certs=False
        )
    
    def create_index(self, dims: Optional[int] = None) -> bool:
        dims = dims or config.embedding_dims
        
        mapping = {
            "mappings": {
                "properties": {
                    "content": {
                        "type": "text"
                    },
                    "embedding": {
                        "type": "dense_vector",
                        "dims": dims,
                        "index": True,
                        "similarity": "cosine"
                    },
                    "metadata": {
                        "properties": {
                            "source": {"type": "keyword"},
                            "header_1": {"type": "keyword"},
                            "header_2": {"type": "keyword"},
                            "header_3": {"type": "keyword"}
                        }
                    },
                    "created_at": {
                        "type": "date"
                    }
                }
            }
        }
        
        if self.es.indices.exists(index=self.index_name):
            return False
        
        self.es.indices.create(index=self.index_name, body=mapping)
        return True
    
    def delete_index(self) -> bool:
        if self.es.indices.exists(index=self.index_name):
            self.es.indices.delete(index=self.index_name)
            return True
        return False
    
    def index_document(self, doc: dict) -> str:
        doc["created_at"] = datetime.utcnow().isoformat()
        response = self.es.index(index=self.index_name, document=doc)
        return response["_id"]
    
    def index_documents(self, documents: list[dict]) -> int:
        actions = []
        for doc in documents:
            action = {
                "_index": self.index_name,
                "_source": {
                    **doc,
                    "created_at": datetime.utcnow().isoformat()
                }
            }
            actions.append(action)
        
        success, _ = bulk(self.es, actions)
        return success
    
    def count_documents(self) -> int:
        response = self.es.count(index=self.index_name)
        return response["count"]
    
    def search_knn(self, query_vector: list[float], k: int = 10) -> list[dict]:
        query = {
            "knn": {
                "field": "embedding",
                "query_vector": query_vector,
                "k": k,
                "num_candidates": k * 10
            }
        }
        
        response = self.es.search(index=self.index_name, body=query)
        
        results = []
        for hit in response["hits"]["hits"]:
            results.append({
                "score": hit["_score"],
                "content": hit["_source"]["content"],
                "metadata": hit["_source"].get("metadata", {})
            })
        
        return results
    
    def test_connection(self) -> bool:
        try:
            self.es.ping()
            return True
        except Exception:
            return False
