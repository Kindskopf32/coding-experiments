from __future__ import annotations

from openai import OpenAI, AuthenticationError, RateLimitError, APIConnectionError

from config import Config
from retriever import SearchResult

_SYSTEM_PROMPT = """\
You are a helpful assistant that answers questions based on the provided context.
- Answer using only the information in the context below.
- Cite sources using [Chunk N] notation, where N is the chunk number shown in the context and include the source file.
- If the context does not contain enough information, say so clearly.
- Be concise and accurate."""


def _format_context(results: list[SearchResult]) -> str:
    parts: list[str] = []
    for i, r in enumerate(results, start=1):
        header = f"[Chunk {i}] {r.source_file} | {r.header_path} (score: {r.score:.3f})"
        parts.append(f"{header}\n{r.content}")
    return "\n\n---\n\n".join(parts)


def generate(question: str, results: list[SearchResult], cfg: Config) -> str:
    if not results:
        return "No relevant context found. Try indexing documents first."

    client = OpenAI(
        api_key=cfg.openrouter_api_key,
        base_url=cfg.openrouter_base_url,
    )

    context = _format_context(results)
    user_message = f"Context:\n\n{context}\n\nQuestion: {question}"

    try:
        response = client.chat.completions.create(
            model=cfg.openrouter_model,
            messages=[
                {"role": "system", "content": _SYSTEM_PROMPT},
                {"role": "user", "content": user_message},
            ],
            temperature=0.1,
            max_tokens=1024,
        )
    except AuthenticationError:
        raise RuntimeError(
            "OpenRouter authentication failed. Check that OPENROUTER_API_KEY is valid."
        )
    except RateLimitError:
        raise RuntimeError(
            "OpenRouter rate limit exceeded. Wait a moment and try again."
        )
    except APIConnectionError as exc:
        raise RuntimeError(
            f"Could not connect to OpenRouter at {cfg.openrouter_base_url}. "
            f"Check your network connection.\nDetails: {exc}"
        )

    return response.choices[0].message.content or ""
