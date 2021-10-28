FEATURES_MINIMAL = --no-default-features --features 'std cargo' -p clap:3.0.0-beta.5
FEATURES_FULL = --features 'wrap_help yaml regex unstable-replace unstable-multicall unstable-grouped'

.PHONY: features-minimal features-full lint-minimal lint-full fmt-check lint coverage bench tests test debug clean

features-minimal:
	@echo "$(FEATURES_MINIMAL)"

features-full:
	@echo "$(FEATURES_FULL)"

lint-minimal:
	cargo clippy $(FEATURES_MINIMAL) -- -D warnings

lint-full:
	cargo clippy $(FEATURES_FULL) -- -D warnings

fmt-check:
	cargo fmt -- --check

lint: lint-minimal lint-full fmt-check

coverage:
	cargo llvm-cov $(FEATURES_FULL) --lcov --output-path lcov.info

bench:
	cargo bench -- --output-format bencher

tests:
	cargo test $(FEATURES_FULL)

test:
ifeq (3, $(words $(MAKECMDGOALS)))
	cargo test $(FEATURES_FULL) --test $(word 2,$(MAKECMDGOALS)) -- $(word 3,$(MAKECMDGOALS))
else
	cargo test $(FEATURES_FULL) --test $(word 2,$(MAKECMDGOALS))
endif

debug:
ifeq (3, $(words $(MAKECMDGOALS)))
	cargo test $(FEATURES_FULL) --features debug --test $(word 2,$(MAKECMDGOALS)) -- $(word 3,$(MAKECMDGOALS)) --nocapture
else
	cargo test $(FEATURES_FULL) --features debug --test $(word 2,$(MAKECMDGOALS)) -- --nocapture
endif

clean:
	cargo clean
	find . -type f -name "*.orig" -exec rm {} \;
	find . -type f -name "*.bk" -exec rm {} \;
	find . -type f -name ".*~" -exec rm {} \;
