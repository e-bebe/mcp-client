## test

echo '{"jsonrpc": "2.0", "method": "callTool", "params": {"name": "search_repositories", "params": {"query": "language:rust", "page": 1, "per_page": 10}}, "id": 1}' | cargo run
