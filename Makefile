validate:
	@$(MAKE) -s format
	@$(MAKE) -s check
	@$(MAKE) -s lint
	@$(MAKE) -s test

check:
	@cargo check --workspace --all-targets

lint:
	@cargo clippy --workspace --all-targets

test:
	@cargo test --release --workspace --all-targets

format:
	@cargo fmt --all
