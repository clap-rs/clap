THIS_MAKEFILE_PATH:=$(word $(words $(MAKEFILE_LIST)),$(MAKEFILE_LIST))
THIS_DIR:=$(shell cd $(dir $(THIS_MAKEFILE_PATH));pwd)

test:
	cargo test

build:
	cargo build

update-readme:
	cd "$(THIS_DIR)"
	cp src/lib.rs code.bak
	cat README.md | sed -e 's/^/\/\/! /g' > readme.bak
	sed -i '/\/\/!/d' src/lib.rs
	sed -i '/\/\/ DOCS/r readme.bak' src/lib.rs
	cat src/lib.rs | sed -e 's/`rust/`ignore/g' > src/lib.rs.tmp
	cat src/lib.rs.tmp | sed -e 's/`toml/`ignore/g' > src/lib.rs
	cat src/lib.rs | sed -e 's/\`sh/`ignore/g' > src/lib.rs.tmp
	make clean || (make clean && false)

clean:
	cd "$(THIS_DIR)"
	mv code.bak src/lib.rs || true
	rm src/lib.rs.t* || true
	rm *.bak || true
