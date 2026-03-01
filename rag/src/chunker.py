from pathlib import Path
from typing import Generator
from langchain_text_splitters import MarkdownHeaderTextSplitter, RecursiveCharacterTextSplitter
from dataclasses import dataclass


@dataclass
class Chunk:
    content: str
    metadata: dict


class MarkdownChunker:
    def __init__(self, chunk_size: int = 1000, chunk_overlap: int = 200):
        self.headers_to_split_on = [
            ("#", "header_1"),
            ("##", "header_2"),
            ("###", "header_3"),
        ]
        self.markdown_splitter = MarkdownHeaderTextSplitter(
            headers_to_split_on=self.headers_to_split_on,
            strip_headers=False
        )
        self.text_splitter = RecursiveCharacterTextSplitter(
            chunk_size=chunk_size,
            chunk_overlap=chunk_overlap,
            separators=["\n\n", "\n", ". ", " ", ""]
        )
    
    def chunk_file(self, file_path: Path) -> list[Chunk]:
        content = file_path.read_text(encoding="utf-8")
        return self.chunk_text(content, str(file_path))
    
    def chunk_text(self, text: str, source: str) -> list[Chunk]:
        md_chunks = self.markdown_splitter.split_text(text)
        
        chunks = []
        for md_chunk in md_chunks:
            chunk_metadata = dict(md_chunk.metadata)
            chunk_metadata["source"] = source
            
            if len(md_chunk.page_content) > self.text_splitter._chunk_size:
                sub_chunks = self.text_splitter.split_text(md_chunk.page_content)
                for sub_chunk in sub_chunks:
                    chunks.append(Chunk(content=sub_chunk, metadata=chunk_metadata.copy()))
            else:
                chunks.append(Chunk(content=md_chunk.page_content, metadata=chunk_metadata))
        
        return chunks
    
    def chunk_directory(self, dir_path: Path) -> Generator[tuple[Path, list[Chunk]], None, None]:
        dir_path = Path(dir_path)
        for md_file in dir_path.rglob("*.md"):
            chunks = self.chunk_file(md_file)
            yield md_file, chunks
