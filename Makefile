build:
	cargo build

clean:
	cargo clean

lint: 
	cargo fmt --check
	cargo clippy

run:
	cargo run

# TEMP 
list: 
	cargo run list

# watch_list:
# 	watch -n 1 uv run python src/main.py list
