# DuckDB owns its ABI, platform footer, static linking and extension suffix.
get_filename_component(THINKTHEN_ROOT "${CMAKE_CURRENT_SOURCE_DIR}/../.." ABSOLUTE)
if(NOT THINKTHEN_ARCHIVE)
    find_program(CARGO_EXECUTABLE cargo REQUIRED)
    set(rust_target "${CMAKE_CURRENT_BINARY_DIR}/rust")
    set(cargo_target_args)
    set(archive_dir "${rust_target}/release")
    if(Rust_CARGO_TARGET)
        list(APPEND cargo_target_args --target ${Rust_CARGO_TARGET})
        set(archive_dir "${rust_target}/${Rust_CARGO_TARGET}/release")
    endif()
    set(THINKTHEN_ARCHIVE "${archive_dir}/libthinkthen_ffi.a")
    # Cargo tracks all Rust inputs, including the shared SQL engine and lockfile.
    add_custom_target(thinkthen_rust
        COMMAND ${CMAKE_COMMAND} -E env "CARGO_TARGET_DIR=${rust_target}"
            "RUSTFLAGS=$ENV{RUSTFLAGS} -C relocation-model=pic"
            ${CARGO_EXECUTABLE} build --locked --release -p thinkthen-ffi ${cargo_target_args}
        WORKING_DIRECTORY "${THINKTHEN_ROOT}"
        BYPRODUCTS "${THINKTHEN_ARCHIVE}" VERBATIM)
endif()
build_static_extension(thinkthen src/extension.cpp)
build_loadable_extension(thinkthen "" src/extension.cpp)
foreach(target thinkthen_extension thinkthen_loadable_extension)
    if(TARGET thinkthen_rust)
        add_dependencies(${target} thinkthen_rust)
    endif()
    target_include_directories(${target} PRIVATE "${CMAKE_CURRENT_BINARY_DIR}"
        "${THINKTHEN_ROOT}/crates/thinkthen-ffi/include")
    target_compile_options(${target} PRIVATE -Wall -Wextra -Werror -Wno-unused-parameter)
    target_link_libraries(${target} "${THINKTHEN_ARCHIVE}")
    if(APPLE)
        execute_process(COMMAND sh "${THINKTHEN_ROOT}/scripts/native-frameworks" --cmake
            OUTPUT_VARIABLE frameworks OUTPUT_STRIP_TRAILING_WHITESPACE COMMAND_ERROR_IS_FATAL ANY)
        target_link_libraries(${target} ${frameworks} iconv)
    elseif(UNIX)
        target_link_libraries(${target} dl pthread m rt util)
    endif()
endforeach()
