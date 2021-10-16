FEATURES = "wrap_help yaml regex unstable-replace unstable-multicall unstable-grouped"

.PHONY: lint-minimal lint-full fmt-check lint bench tests test debug clean

lint-minimal:
	cargo clippy --no-default-features --features "std cargo" -p clap:3.0.0-beta.5 -- -D warnings

lint-full:
	cargo clippy --features $(FEATURES) -- -D warnings

fmt-check:
	cargo fmt -- --check

lint: lint-minimal lint-full fmt-check

bench:
	cargo bench -- --output-format bencher

tests:
	cargo test --features $(FEATURES)

test:
ifeq (3, $(words $(MAKECMDGOALS)))
	cargo test --features $(FEATURES) --test $(word 2,$(MAKECMDGOALS)) -- $(word 3,$(MAKECMDGOALS))
else
	cargo test --features $(FEATURES) --test $(word 2,$(MAKECMDGOALS))
endif

debug:
ifeq (3, $(words $(MAKECMDGOALS)))
	cargo test --features $(FEATURES) --features debug --test $(word 2,$(MAKECMDGOALS)) -- $(word 3,$(MAKECMDGOALS)) --nocapture
else
	cargo test --features $(FEATURES) --features debug --test $(word 2,$(MAKECMDGOALS)) -- --nocapture
endif

clean:
	cargo clean
	find . -type f -name "*.orig" -exec rm {} \;
	find . -type f -name "*.bk" -exec rm {} \;
	find . -type f -name ".*~" -exec rm {} \;
