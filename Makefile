PROJ_DIR := $(dir $(abspath $(lastword $(MAKEFILE_LIST))))
EXT_NAME=thinkthen
EXT_CONFIG=${PROJ_DIR}extension_config.cmake

include extension-ci-tools/makefiles/duckdb_extension.Makefile

# Downloads belong to CI preparation, never to release or test targets.
.PHONY: thinkthen_prepare_rust
configure_ci: thinkthen_prepare_rust
thinkthen_prepare_rust:
	@if [ "$(LINUX_CI_IN_DOCKER)" != "0" ]; then \
		python3 "$(PROJ_DIR)databases/duckdb/community/prepare.py" "$(OSX_BUILD_ARCH)"; \
	fi
