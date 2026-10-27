# Official recipe: https://github.com/duckdb/extension-template
PROJ_DIR := $(dir $(abspath $(firstword $(MAKEFILE_LIST))))
EXT_NAME=thinkthen
EXT_CONFIG=$(PROJ_DIR)extension_config.cmake
# Local builds may use the verified tools prepared by scripts/duckdb-prepare.
EXTENSION_CI_TOOLS ?= extension-ci-tools
include $(EXTENSION_CI_TOOLS)/makefiles/duckdb_extension.Makefile
TESTS_BASE_DIRECTORY=$(PROJ_DIR)databases/duckdb/test/
