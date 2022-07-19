# CI Steps
#
# Considerations
# - Easy to debug: show the command being run
# - Leverage CI features: Only run individual steps so we can use features like reporting elapsed time per step

ARGS?=--workspace
TOOLCHAIN_TARGET ?=
ifneq (${TOOLCHAIN_TARGET},)
  ARGS+=--target ${TOOLCHAIN_TARGET}
endif

MSRV?=1.46.0

_FEATURES = minimal default wasm full debug release
_FEATURES_minimal = --no-default-features
_FEATURES_default =
_FEATURES_full = --features "wrap_help yaml doc"
_FEATURES_debug = ${_FEATURES_full} --features debug
_FEATURES_release = ${_FEATURES_full} --release

_TARGETS=--lib --tests --examples

check-%:
	cargo check ${_FEATURES_${@:check-%=%}} ${_TARGETS} ${ARGS}

build-%:
	cargo test ${_FEATURES_${@:build-%=%}} ${_TARGETS} --no-run ${ARGS}

test-%:
	cargo test ${_FEATURES_${@:test-%=%}} ${ARGS}

clippy-%:
	cargo clippy ${_FEATURES_${@:clippy-%=%}} ${ARGS} ${_TARGETS} -- -D warnings -A deprecated

doc:
	cargo doc --workspace --all-features --no-deps --document-private-items
