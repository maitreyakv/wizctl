build:
	uv venv
	uv sync

clean-mypy:
	rm -rf .mpypy_cache

clean-venv:	
	rm -rf .venv

clean: clean-mypy clean-venv

lint: 
	uv run ruff check
	uv run mypy src/

watch_ports:
	watch -n 1 "ss -at -u '( dport = :38899 or sport = :38899 )'"

# TEMP 
list: 
	uv run python src/main.py list

watch_list:
	watch -n 1 uv run python src/main.py list
