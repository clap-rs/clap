# CI Steps
#
# Considerations
# - Easy to debug: show the command being run
# - Leverage CI features: Only run individual steps so we can use features like reporting elapsed time per step

TOOLCHAIN_TARGET ?=
ifneq (${TOOLCHAIN_TARGET},)
  _TARGET_ARG =--target ${TOOLCHAIN_TARGET}
endif
_TARGET_ARG ?=

_FEATURES = minimal default wasm full release
_FEATURES_minimal = --no-default-features --features "std cargo"
_FEATURES_default =
_FEATURES_wasm = --features "yaml regex unstable-replace"
_FEATURES_full = --features "yaml regex unstable-replace wrap_help"
_FEATURES_debug = ${_FEATURES_full} --features debug
_FEATURES_release = ${_FEATURES_full} --release

check-%:
	cargo check --all-targets ${_RELEASE_ARG} ${_TARGET_ARG} ${_FEATURES_${@:check-%=%}}

build-%:
	cargo test --no-run ${_RELEASE_ARG} ${_TARGET_ARG} ${_FEATURES_${@:build-%=%}}

test-%:
	cargo test ${_RELEASE_ARG} ${_TARGET_ARG} ${_FEATURES_${@:test-%=%}}
