.PHONY: check

check:
	cargo fmt && \
	cargo clippy -- \
		-W clippy::pedantic \
		-W clippy::nursery
