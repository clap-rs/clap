THIS_MAKEFILE_PATH:=$(word $(words $(MAKEFILE_LIST)),$(MAKEFILE_LIST))
THIS_DIR:=$(shell cd $(dir $(THIS_MAKEFILE_PATH));pwd)
URL:=
BRANCH:=

build:
	cargo update
	cargo build --release

test:
	cd "$(THIS_DIR)"
	cp Cargo.toml cargo.bak
	@read -p "Fork URL: " URL; \
	sed "/^git/c\git = \"$$URL\"" cargo.bak > Cargo.toml
	@read -p "Branch [master]: " BRANCH; \
	$(if $(strip $(BRANCH)), \
		sed -i "/^branch/c\branch = \"master\"" Cargo.toml, \
		sed -i "/^branch/c\branch = \"$$BRANCH\"" Cargo.toml)
	(make clean) || (make clean && false)
	cd "$(THIS_DIR)"
	(make build) || (make clean && false)
	cd "$(THIS_DIR)"
	cargo test
	(./run_tests.py) || (make clean && false)
	mv cargo.bak Cargo.toml
	make clean


clean:
	cd "$(THIS_DIR)"
	cargo clean
