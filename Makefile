build:
	uv venv
	uv sync

clean-mypy:
	rm -rf .mpypy_cache

clean-venv:	
	rm -rf .venv

clean: clean-mypy clean-venv

lint: build
	uv run ruff check
	uv run mypy src/

# TEMP 
list: build
	uv run python src/main.py list
