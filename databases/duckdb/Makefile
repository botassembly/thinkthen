.PHONY: clean clean_all

PROJ_DIR := $(dir $(abspath $(lastword $(MAKEFILE_LIST))))

EXTENSION_NAME=thinkthen

# Set to 1 to enable Unstable API (binaries will only work on TARGET_DUCKDB_VERSION, forwards compatibility will be broken)
# Note: currently extension-template-rs requires this, as duckdb-rs relies on unstable C API functionality
USE_UNSTABLE_C_API=1

# Target DuckDB version: the one pin is tools/version.env (review 4:
# the Makefile kept its own copy of the same constant).
TARGET_DUCKDB_VERSION=$(shell sed -n 's/^DUCKDB_VERSION=//p' tools/version.env)

all: configure debug

# Include makefiles from DuckDB
include extension-ci-tools/makefiles/c_api_extensions/base.Makefile
include extension-ci-tools/makefiles/c_api_extensions/rust.Makefile

# The vendored makefile's cargo build carries no lock. Its one flag
# variable reaches both build lines, so the lock rides on it
# (surfaces-review-5). scripts/check_locked_calls.py checks this line.
CARGO_OVERRIDE_DUCKDB_RS_FLAG += --locked

configure: venv platform extension_version

debug: build_extension_library_debug build_extension_with_metadata_debug
release: build_extension_library_release build_extension_with_metadata_release

test: test_debug
test_debug: test_extension_debug
test_release: test_extension_release

clean: clean_build clean_rust
clean_all: clean_configure clean
