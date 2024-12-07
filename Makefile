build:
	uv venv

clean-mypy:
	rm -rf .mpypy_cache

clean-venv:	
	rm -rf .venv

clean: clean-mypy clean-venv

lint: build
	uv run ruff check
	uv run mypy src/
