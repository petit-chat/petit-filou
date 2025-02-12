.PHONY: test
test:
	cargo test --no-run --locked
	cargo test -- --nocapture --quiet

.PHONY: lint
lint:
	cargo fmt --all -- --check
	cargo clippy --all-targets -- -D warnings
	RUSTDOCFLAGS="-D warnings" cargo doc

.PHONY: build
build:
	cargo build --release
