prepare:
	rustup target add wasm32-unknown-unknown

build-contract:
	cargo build --release -p casper-contract-eip-1337 --target wasm32-unknown-unknown
	wasm-strip target/wasm32-unknown-unknown/release/*.wasm

test-only:
	cargo test -p tests

copy-wasm-file-to-test:
	mkdir -p tests/wasm
	cp target/wasm32-unknown-unknown/release/*.wasm tests/wasm
	cp tests/erc-20-wasm/erc-20-e973bb5.wasm tests/wasm/erc-20.wasm

test: build-contract copy-wasm-file-to-test test-only

clippy:
	cargo clippy --all-targets --all -- -D warnings -A renamed_and_removed_lints

check-lint: clippy
	cargo fmt --all -- --check

lint: clippy
	cargo fmt --all
	
clean:
	cargo clean
	rm tests/wasm/*
