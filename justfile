run-test TESTG TEST="":
	cargo test --test {{TESTG}} -- {{TEST}}

debug TESTG TEST="":
	cargo test --test {{TESTG}} --features debug -- {{TEST}}

run-tests:
	cargo test --features "yaml unstable"

@bench:
	cargo bench

@lint:
	rustup component add clippy
	rustup component add rustfmt
	cargo clippy --lib --features "yaml unstable" -- -D warnings
	cargo clippy --tests --examples --features "yaml unstable"
	cargo fmt -- --check

clean:
	cargo clean
	find . -type f -name "*.orig" -exec rm {} \;
	find . -type f -name "*.bk" -exec rm {} \;
	find . -type f -name ".*~" -exec rm {} \;

top-errors NUM="95":
	@cargo check 2>&1 | head -n {{NUM}}

count-errors:
	@cargo check 2>&1 | grep -e '^error' | wc -l

find-errors:
	@cargo check 2>&1 | grep --only-matching -e '-->[^:]*' | sort | uniq -c | sort -nr

count-warnings:
	@cargo check 2>&1 | grep -e '^warning' | wc -l

find-warnings:
	@cargo check 2>&1 | grep -A1 -e 'warning' | grep --only-matching -e '-->[^:]*' | sort | uniq -c | sort -nr

@count-failures:
	./count-tests.sh
