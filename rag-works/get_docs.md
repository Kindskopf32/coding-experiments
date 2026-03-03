# Getting docs with the provided list.txt

Use docling to pull a list of URLs into markdown files
```bash
while IFS= read -r URL; do echo "Pulling $URL" && uvx docling "$URL"; done < list.txt
```
