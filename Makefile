CARGO_BINARY_ARGS = --bin=wizctl --features=cli

install:
	cargo install ${CARGO_BINARY_ARGS} --path=${PWD}

uninstall:
	cargo uninstall

build:
	cargo build ${CARGO_BINARY_ARGS}

check:
	cargo check ${CARGO_BINARY_ARGS}

lint: 
	cargo fmt --check
	cargo clippy ${CARGO_BINARY_ARGS} -- -D warnings

clean:
	cargo clean
