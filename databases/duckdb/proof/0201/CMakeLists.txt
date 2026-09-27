cmake_minimum_required(VERSION 3.15)
project(thinkthen_probe LANGUAGES CXX)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

if(NOT EXISTS "${THINKTHEN_PROBE_RUST_STATICLIB}")
  message(FATAL_ERROR "Set THINKTHEN_PROBE_RUST_STATICLIB to the compiled Rust proof library")
endif()

build_loadable_extension(thinkthen_probe " " src/probe.cpp)
target_link_libraries(thinkthen_probe_loadable_extension "${THINKTHEN_PROBE_RUST_STATICLIB}")

# Link the official v1.5.5 release archives instead of building DuckDB from source.
# The CMake caller verifies the release ZIP digest before setting this directory.
if(NOT EXISTS "${THINKTHEN_PROBE_DUCKDB_STATIC_DIR}/libduckdb_static.a")
  message(FATAL_ERROR "Set THINKTHEN_PROBE_DUCKDB_STATIC_DIR to the verified v1.5.5 archives")
endif()
file(GLOB DUCKDB_PROBE_ARCHIVES "${THINKTHEN_PROBE_DUCKDB_STATIC_DIR}/lib*.a")
target_link_libraries(thinkthen_probe_loadable_extension
  -Wl,--start-group ${DUCKDB_PROBE_ARCHIVES} -Wl,--end-group dl pthread)
target_link_options(thinkthen_probe_loadable_extension PRIVATE
  -Wl,--gc-sections -Wl,--exclude-libs,ALL)
