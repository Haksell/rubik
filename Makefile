all:
	@$(MAKE) check
	@$(MAKE) lint

check:
	cargo check --workspace --all-targets

lint:
	cargo clippy --workspace --all-targets