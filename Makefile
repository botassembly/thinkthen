PROJ_DIR := $(dir $(abspath $(lastword $(MAKEFILE_LIST))))
EXT_NAME=thinkthen
EXT_CONFIG=${PROJ_DIR}extension_config.cmake

# The immutable source ref differs from the native ABI's short source identifier.
# Upstream 2.0 forwards its checkout ref as DUCKDB_VERSION; use its supported
# explicit version input only for the authority's approved full source ref.
THINKTHEN_INPUTS := $(dir $(abspath $(firstword $(MAKEFILE_LIST))))databases/duckdb/tools/inputs.py
THINKTHEN_ABI_OVERRIDE := $(shell python3 "$(THINKTHEN_INPUTS)" abi-override)
ifneq ($(THINKTHEN_ABI_OVERRIDE),)
EXT_FLAGS += -DDUCKDB_EXPLICIT_VERSION=$(THINKTHEN_ABI_OVERRIDE)
endif

# Community Linux preparation and compilation run in separate containers.
# Keep fetched crates and the pinned toolchain in their shared mounted checkout.
ifeq ($(LINUX_CI_IN_DOCKER),1)
export CARGO_HOME ?= $(PROJ_DIR)build/community-rust/cargo
export RUSTUP_HOME ?= $(PROJ_DIR)build/community-rust/rustup
endif

include extension-ci-tools/makefiles/duckdb_extension.Makefile

# This repository keeps its extension tests beside the DuckDB adapter.
ifneq ($(SUBSET_EXTENSIONS_TESTS),complete)
TESTS_BASE_DIRECTORY=$(PROJ_DIR)databases/duckdb/community/test/sql/
endif

# Load DONT_LINK builds from DuckDB's local repository; a missing load fails.
export DUCKDB_TEST_AUTOLOADING ?= all
ifeq ($(shell test -f "$(TEST_RUNNER_SCRIPT)" && grep -q '"--test-flags"' "$(TEST_RUNNER_SCRIPT)" && echo yes),yes)
TEST_PATH := $(TEST_PATH) --test-flags="--require thinkthen" --workers 2 --fail-require-skip
else
TEST_PATH := $(TEST_PATH) --require thinkthen
endif

# Downloads belong to CI preparation, never to release or test targets.
.PHONY: thinkthen_prepare_rust
configure_ci: thinkthen_prepare_rust
thinkthen_prepare_rust:
	@if [ "$(LINUX_CI_IN_DOCKER)" != "0" ]; then \
		python3 "$(PROJ_DIR)databases/duckdb/community/prepare.py" "$(OSX_BUILD_ARCH)"; \
	fi
