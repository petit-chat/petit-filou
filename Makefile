.PHONY: coverage
coverage:
	cargo llvm-cov --open

.PHONY: test
test:
	cargo test --all-features

.PHONY: lint
lint:
	cargo fmt --all -- --check
	cargo clippy --all-targets -- -D warnings
	RUSTDOCFLAGS="-D warnings" cargo doc

.PHONY: build
build:
	cargo build --release
