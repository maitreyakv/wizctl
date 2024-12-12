build:
	cargo build

clean:
	cargo clean

lint: 
	cargo fmt --check
	cargo clippy
