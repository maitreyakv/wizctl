build:
	cargo build

clean:
	cargo clean

lint: 
	cargo fmt --check

watch_ports:
	watch -n 1 "ss -at -u '( dport = :38899 or sport = :38899 )'"

# TEMP 
# list: 
# 	uv run python src/main.py list
#
# watch_list:
# 	watch -n 1 uv run python src/main.py list
