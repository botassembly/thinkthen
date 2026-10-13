file(STRINGS "${CMAKE_CURRENT_LIST_DIR}/crates/thinkthen/Cargo.toml" THINKTHEN_VERSION_LINE
  REGEX "^version = \"[^\"]+\"$")
list(GET THINKTHEN_VERSION_LINE 0 THINKTHEN_VERSION_LINE)
string(REGEX REPLACE "^version = \"([^\"]+)\"$" "\\1" THINKTHEN_EXTENSION_VERSION "${THINKTHEN_VERSION_LINE}")

# DuckDB 2.0 links only explicitly selected extensions and rejects DONT_LINK.
set(THINKTHEN_LINK_FLAGS)
if(NOT COMMAND duckdb_extension_statically_link)
  list(APPEND THINKTHEN_LINK_FLAGS DONT_LINK)
endif()

duckdb_extension_load(thinkthen
  SOURCE_DIR "${CMAKE_CURRENT_LIST_DIR}"
  INCLUDE_DIR "${CMAKE_CURRENT_LIST_DIR}/databases/duckdb/cpp/src"
  ${THINKTHEN_LINK_FLAGS}
  LOAD_TESTS
  TEST_DIR "${CMAKE_CURRENT_LIST_DIR}/databases/duckdb/community/test/sql"
  EXTENSION_VERSION "${THINKTHEN_EXTENSION_VERSION}")
