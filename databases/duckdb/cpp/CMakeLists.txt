cmake_minimum_required(VERSION 3.15)
project(thinkthen LANGUAGES CXX)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

if(NOT EXISTS "${THINKTHEN_RUST_STATICLIB}")
  message(FATAL_ERROR "Set THINKTHEN_RUST_STATICLIB to the checked Rust bridge archive")
endif()
if(NOT EXISTS "${THINKTHEN_DUCKDB_STATIC_DIR}/libduckdb_static.a")
  message(FATAL_ERROR "Set THINKTHEN_DUCKDB_STATIC_DIR to the selected verified release archives")
endif()

execute_process(COMMAND python3 "${CMAKE_CURRENT_LIST_DIR}/../../../sdlc/scripts/release-archive-tree.py"
  --git "${CMAKE_SOURCE_DIR}" rev-parse HEAD
  OUTPUT_VARIABLE DUCKDB_SOURCE_COMMIT OUTPUT_STRIP_TRAILING_WHITESPACE
  RESULT_VARIABLE DUCKDB_SOURCE_STATUS)
if(NOT DUCKDB_SOURCE_STATUS EQUAL 0 OR
   NOT DUCKDB_SOURCE_COMMIT STREQUAL THINKTHEN_DUCKDB_SOURCE_COMMIT)
  message(FATAL_ERROR "DuckDB C++ source must be the selected pinned commit")
endif()

if(NOT IS_ABSOLUTE "${THINKTHEN_DUCKDB_ARCHIVE_MANIFEST}" OR
   NOT EXISTS "${THINKTHEN_DUCKDB_ARCHIVE_MANIFEST}")
  message(FATAL_ERROR "Set THINKTHEN_DUCKDB_ARCHIVE_MANIFEST to the selected manifest")
endif()
file(STRINGS "${THINKTHEN_DUCKDB_ARCHIVE_MANIFEST}" ARCHIVE_MANIFEST)
file(GLOB ARCHIVE_FILES "${THINKTHEN_DUCKDB_STATIC_DIR}/lib*.a")
list(LENGTH ARCHIVE_FILES ARCHIVE_FILE_COUNT)
list(LENGTH ARCHIVE_MANIFEST ARCHIVE_MANIFEST_COUNT)
if(NOT ARCHIVE_FILE_COUNT EQUAL ARCHIVE_MANIFEST_COUNT)
  message(FATAL_ERROR "DuckDB static archive set differs from the fixed manifest")
endif()
set(DUCKDB_ARCHIVES)
set(ARCHIVE_NAMES)
foreach(LINE IN LISTS ARCHIVE_MANIFEST)
  string(REPLACE " " ";" PARTS "${LINE}")
  list(LENGTH PARTS PART_COUNT)
  if(NOT PART_COUNT EQUAL 2)
    message(FATAL_ERROR "Malformed DuckDB archive manifest line")
  endif()
  list(GET PARTS 0 EXPECTED)
  list(GET PARTS 1 NAME)
  string(LENGTH "${EXPECTED}" HASH_LENGTH)
  if(NOT HASH_LENGTH EQUAL 64 OR NOT EXPECTED MATCHES "^[0-9a-f]+$" OR
     NOT NAME MATCHES "^lib[A-Za-z0-9_.-]+\\.a$" OR NAME IN_LIST ARCHIVE_NAMES)
    message(FATAL_ERROR "Malformed or duplicate DuckDB archive manifest line")
  endif()
  list(APPEND ARCHIVE_NAMES "${NAME}")
  set(ARCHIVE "${THINKTHEN_DUCKDB_STATIC_DIR}/${NAME}")
  if(NOT EXISTS "${ARCHIVE}")
    message(FATAL_ERROR "DuckDB archive ${NAME} is missing")
  endif()
  file(SHA256 "${ARCHIVE}" OBSERVED)
  if(NOT OBSERVED STREQUAL EXPECTED)
    message(FATAL_ERROR "DuckDB archive ${NAME} differs from the pinned release")
  endif()
  list(APPEND DUCKDB_ARCHIVES "${ARCHIVE}")
endforeach()

build_loadable_extension(thinkthen " " src/thinkthen.cpp src/find.cpp src/scalar_owner.cpp src/scalar_settings.cpp src/portable.cpp src/plan.cpp
  src/listed_result.cpp src/nested.cpp src/nested_result.cpp src/usage.cpp src/removed.cpp
  src/relate.cpp src/relate_query.cpp)
if(APPLE)
  # Hide every name in the Rust bridge archive, as --exclude-libs does on Linux.
  # Its Rust standard library, panic hook and bundled SQLite stay out of the
  # export table, so no other image can bind to them or look them up (ticket 0388).
  target_link_options(thinkthen_loadable_extension PRIVATE "LINKER:-load_hidden,${THINKTHEN_RUST_STATICLIB}")
  target_link_libraries(thinkthen_loadable_extension ${DUCKDB_ARCHIVES})
else()
  target_link_libraries(thinkthen_loadable_extension "${THINKTHEN_RUST_STATICLIB}")
  target_link_libraries(thinkthen_loadable_extension
    -Wl,--start-group ${DUCKDB_ARCHIVES} -Wl,--end-group dl pthread)
  target_link_options(thinkthen_loadable_extension PRIVATE
    -Wl,--gc-sections -Wl,--exclude-libs,ALL)
endif()
