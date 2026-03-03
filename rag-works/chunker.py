from __future__ import annotations

import re
from dataclasses import dataclass
from pathlib import Path

import tiktoken

_ENCODER = tiktoken.get_encoding("cl100k_base")
_MAX_TOKENS = 512

# Matches lines starting with ## or ### (section headers)
_HEADER_RE = re.compile(r"^(#{2,3})\s+(.+)$", re.MULTILINE)


@dataclass
class Chunk:
    content: str
    source_file: str
    header_path: str
    chunk_index: int


def _count_tokens(text: str) -> int:
    return len(_ENCODER.encode(text))


def _split_by_paragraphs(header_prefix: str, body: str) -> list[str]:
    """Split body by blank lines into paragraphs, prepending header_prefix to each chunk."""
    paragraphs = re.split(r"\n{2,}", body.strip())
    chunks: list[str] = []
    current_parts: list[str] = []
    current_tokens = _count_tokens(header_prefix)

    for para in paragraphs:
        para = para.strip()
        if not para:
            continue
        para_tokens = _count_tokens(para)
        if current_parts and current_tokens + para_tokens + 1 > _MAX_TOKENS:
            chunks.append(header_prefix + "\n\n" + "\n\n".join(current_parts))
            current_parts = [para]
            current_tokens = _count_tokens(header_prefix) + para_tokens
        else:
            current_parts.append(para)
            current_tokens += para_tokens + 1  # +1 for newline separator

    if current_parts:
        chunks.append(header_prefix + "\n\n" + "\n\n".join(current_parts))

    return chunks if chunks else [header_prefix]


def chunk_file(path: Path) -> list[Chunk]:
    text = path.read_text(encoding="utf-8")
    source_file = path.name
    chunks: list[Chunk] = []
    chunk_index = 0

    # Find all header positions
    header_matches = list(_HEADER_RE.finditer(text))

    if not header_matches:
        # No headers — treat whole file as one section
        sections = [(None, text)]
    else:
        sections: list[tuple[re.Match | None, str]] = []
        # Content before first header
        pre = text[: header_matches[0].start()].strip()
        if pre:
            sections.append((None, pre))
        for i, match in enumerate(header_matches):
            end = header_matches[i + 1].start() if i + 1 < len(header_matches) else len(text)
            body = text[match.end() : end]
            sections.append((match, body))

    for match, body in sections:
        if match is None:
            header_line = ""
            header_path = "(preamble)"
        else:
            header_line = match.group(0)
            header_path = match.group(2).strip()

        full_text = (header_line + "\n" + body).strip() if header_line else body.strip()

        if not full_text:
            continue

        if _count_tokens(full_text) <= _MAX_TOKENS:
            chunks.append(
                Chunk(
                    content=full_text,
                    source_file=source_file,
                    header_path=header_path,
                    chunk_index=chunk_index,
                )
            )
            chunk_index += 1
        else:
            # Split oversized section by paragraphs
            sub_chunks = _split_by_paragraphs(header_line, body)
            for sub in sub_chunks:
                chunks.append(
                    Chunk(
                        content=sub.strip(),
                        source_file=source_file,
                        header_path=header_path,
                        chunk_index=chunk_index,
                    )
                )
                chunk_index += 1

    return chunks
