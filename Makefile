all: lint build build-release

build:
	cargo build

build-release:
	cargo build --release

check:
	cargo check

lint: 
	cargo fmt --check
	cargo clippy -- -D warnings

clean:
	cargo clean
