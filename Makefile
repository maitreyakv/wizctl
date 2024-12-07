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

# TEMP 
list: 
	uv run python src/main.py list
