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

_FEATURES = minimal default wasm full release
_FEATURES_minimal = --no-default-features --features "std"
_FEATURES_default =
_FEATURES_wasm = --features "derive cargo env unicode yaml regex unstable-replace unstable-multicall unstable-grouped"
_FEATURES_full = --features "derive cargo env unicode yaml regex unstable-replace unstable-multicall unstable-grouped wrap_help doc"
_FEATURES_debug = ${_FEATURES_full} --features debug
_FEATURES_release = ${_FEATURES_full} --release

check-%:
	cargo check ${_FEATURES_${@:check-%=%}} --all-targets ${ARGS}

build-%:
	cargo test ${_FEATURES_${@:build-%=%}} --all-targets --no-run ${ARGS}

test-%:
	cargo test ${_FEATURES_${@:test-%=%}} --all-targets ${ARGS}
	cargo test ${_FEATURES_${@:test-%=%}} --doc ${ARGS}
