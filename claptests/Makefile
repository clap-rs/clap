THIS_MAKEFILE_PATH:=$(word $(words $(MAKEFILE_LIST)),$(MAKEFILE_LIST))
THIS_DIR:=$(shell cd $(dir $(THIS_MAKEFILE_PATH));pwd)

build:
	cargo update
	cargo build --release

test:
	cd "$(THIS_DIR)"
	cp Cargo.toml cargo.bak
	read -p "fork url: " FORK
	read -p "branch [master]: " BRANCH
	sed -e "s/\$FORK/$FORK" cargo.bak > Cargo.toml
	if [ "$BRANCH" == "" ]; then
		sed -i "s/\$BRANCH/master/" Cargo.toml
	else
		sed -i "s/\$BRNACH/$BRANCH/" Cargo.toml
	fi
	(make clean) || (make clean && false)
	(make build) || (make clean && false)
	./run_tests.py || (make clean && false)
	echo "All tests pass!"
	mv cargo.bak Cargo.toml
	make clean


clean:
	cd "$(THIS_DIR)"
	cargo clean