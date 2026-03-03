#!/usr/bin/env python3
from __future__ import annotations

import argparse
import sys


def cmd_setup(args: argparse.Namespace) -> None:
    from config import load_config
    from indexer import setup

    cfg = load_config()
    setup(cfg)


def cmd_index(args: argparse.Namespace) -> None:
    from pathlib import Path
    from config import load_config
    from indexer import index_directory

    directory = Path(args.directory)
    if not directory.is_dir():
        print(f"Error: '{args.directory}' is not a directory.", file=sys.stderr)
        sys.exit(1)

    cfg = load_config()
    index_directory(directory, cfg)


def cmd_query(args: argparse.Namespace) -> None:
    from config import load_config
    from retriever import retrieve
    from generator import generate

    cfg = load_config()

    print(f"[query] Retrieving top {cfg.top_k} chunks for: {args.question!r}")
    results = retrieve(args.question, cfg)
    print(f"[query] Received the following elasticsearch response: {results}")

    if not results:
        print("[query] No results found.")
        return

    print(f"[query] Found {len(results)} result(s). Generating answer...\n")
    answer = generate(args.question, results, cfg)
    print(answer)


def main() -> None:
    parser = argparse.ArgumentParser(
        prog="rag",
        description="RAG CLI — index markdown docs and query them with ELSER + OpenRouter",
    )
    subparsers = parser.add_subparsers(dest="command", required=True)

    subparsers.add_parser("setup", help="Create ELSER endpoint, ingest pipeline, and index")

    index_parser = subparsers.add_parser("index", help="Index a directory of markdown files")
    index_parser.add_argument("directory", help="Path to directory containing .md files")

    query_parser = subparsers.add_parser("query", help="Query the indexed documents")
    query_parser.add_argument("question", help="Question to ask")

    args = parser.parse_args()

    try:
        if args.command == "setup":
            cmd_setup(args)
        elif args.command == "index":
            cmd_index(args)
        elif args.command == "query":
            cmd_query(args)
    except ValueError as exc:
        print(f"Configuration error: {exc}", file=sys.stderr)
        sys.exit(1)
    except RuntimeError as exc:
        print(f"Error: {exc}", file=sys.stderr)
        sys.exit(1)
    except KeyboardInterrupt:
        sys.exit(130)


if __name__ == "__main__":
    main()
