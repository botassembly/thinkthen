cmake_minimum_required(VERSION 3.15)
project(thinkthen LANGUAGES CXX)

# Community builds use DuckDB's own static library and metadata machinery.
if(NOT EXTENSION_STATIC_BUILD)
  message(FATAL_ERROR "The community adapter requires EXTENSION_STATIC_BUILD=1")
endif()
if(WIN32 OR EMSCRIPTEN)
  message(FATAL_ERROR "The DuckDB Rust bridge requires native POSIX signals and pipes")
endif()
set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
find_package(Python3 REQUIRED COMPONENTS Interpreter)
set(THINKTHEN_ROOT "${CMAKE_CURRENT_LIST_DIR}")
set(THINKTHEN_COMMUNITY "${THINKTHEN_ROOT}/databases/duckdb/community")

if(NOT Rust_CARGO_TARGET)
  execute_process(COMMAND rustc +1.95.0 -vV OUTPUT_VARIABLE THINKTHEN_RUST_VERSION
    RESULT_VARIABLE THINKTHEN_RUST_STATUS)
  if(NOT THINKTHEN_RUST_STATUS EQUAL 0)
    message(FATAL_ERROR "Prepare Rust 1.95.0 with make configure_ci before building")
  endif()
  string(REGEX MATCH "host: ([^\n]+)" THINKTHEN_RUST_HOST "${THINKTHEN_RUST_VERSION}")
  set(Rust_CARGO_TARGET "${CMAKE_MATCH_1}")
endif()
set(THINKTHEN_RUST_OUT "${CMAKE_CURRENT_BINARY_DIR}/rust")
set(THINKTHEN_RUST_STATICLIB
  "${THINKTHEN_RUST_OUT}/${Rust_CARGO_TARGET}/release/libthinkthen_duckdb_bridge.a")
set(THINKTHEN_NATIVE_LIBS "${THINKTHEN_RUST_OUT}/native-libs.rsp")
set(THINKTHEN_RUST_ARGS --target "${Rust_CARGO_TARGET}" --output "${THINKTHEN_RUST_OUT}"
  "--cc=${CMAKE_C_COMPILER}" "--ar=${CMAKE_AR}" "--cflags=${CMAKE_C_FLAGS}"
  "--sysroot=${CMAKE_OSX_SYSROOT}" "--deployment=${CMAKE_OSX_DEPLOYMENT_TARGET}")
add_custom_target(thinkthen_community_rust
  COMMAND "${Python3_EXECUTABLE}" "${THINKTHEN_COMMUNITY}/build_rust.py" ${THINKTHEN_RUST_ARGS}
  BYPRODUCTS "${THINKTHEN_RUST_STATICLIB}" "${THINKTHEN_NATIVE_LIBS}"
  WORKING_DIRECTORY "${THINKTHEN_ROOT}"
  COMMENT "Build the pinned Rust bridge offline"
  VERBATIM)

include("${THINKTHEN_ROOT}/databases/duckdb/cpp/sources.cmake")
build_loadable_extension(thinkthen " " ${THINKTHEN_CPP_SOURCES})
target_include_directories(thinkthen_loadable_extension PRIVATE
  "${THINKTHEN_ROOT}/databases/duckdb/cpp/src")
add_dependencies(thinkthen_loadable_extension thinkthen_community_rust)
if(APPLE)
  target_link_options(thinkthen_loadable_extension PRIVATE
    "LINKER:-load_hidden,${THINKTHEN_RUST_STATICLIB}")
else()
  target_link_libraries(thinkthen_loadable_extension "${THINKTHEN_RUST_STATICLIB}")
  target_link_options(thinkthen_loadable_extension PRIVATE "LINKER:--exclude-libs,ALL")
endif()
# Rust reports the bridge's actual native dependencies while compiling it.
# Keep them after the archive, including framework pairs on Apple targets.
target_link_libraries(thinkthen_loadable_extension "-Wl,@${THINKTHEN_NATIVE_LIBS}")
set_property(TARGET thinkthen_loadable_extension APPEND PROPERTY LINK_DEPENDS "${THINKTHEN_NATIVE_LIBS}")
