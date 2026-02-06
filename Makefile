nonce:
	cargo run --bin nonce

main:
	cargo clean
	git pull
	cargo run --bin main

build:
	cargo clean
	git pull
	cargo install --path . --root .