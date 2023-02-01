build:  test format
	cargo build

run: build
	./target/debug/pfc

doc: test format
	cargo doc --open

test: format
	cargo test

format:
	cargo fmt --all
