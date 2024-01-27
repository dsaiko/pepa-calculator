build: clean codecheck test
	cargo doc
	cargo build --release

clean:
	cargo clean

codecheck:
	cargo clippy --all-features --tests

test:
	cargo test

fmt:
	cargo fmt

fix:
	cargo fix --allow-dirty

