.PHONY: build test clean

build:
	# It may be that this project was never built, and no Cargo.lock exists.
	# Thus it may be ignored
	cargo update 2>/dev/null || :
	cargo build --release

test:
	$(MAKE) build || ($(MAKE) clean && false)
	./run_tests.py

clean:
	cargo clean
