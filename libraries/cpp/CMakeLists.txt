cmake_minimum_required(VERSION 3.18)
project(thinkthen_cpp VERSION 0.2.0 LANGUAGES CXX)
include(GNUInstallDirs)
include(CMakePackageConfigHelpers)
if(NOT THINKTHEN_NATIVE_SHARED OR NOT THINKTHEN_NATIVE_STATIC OR NOT THINKTHEN_C_HEADER)
  message(FATAL_ERROR "Set THINKTHEN_NATIVE_SHARED, THINKTHEN_NATIVE_STATIC and THINKTHEN_C_HEADER to matching native artifacts")
endif()
file(WRITE "${CMAKE_CURRENT_BINARY_DIR}/client.cpp" "#include <thinkthen/native_implementation.hpp>\n")
add_library(thinkthen_cpp_objects OBJECT "${CMAKE_CURRENT_BINARY_DIR}/client.cpp")
target_compile_features(thinkthen_cpp_objects PRIVATE cxx_std_17)
target_include_directories(thinkthen_cpp_objects PRIVATE include "${CMAKE_CURRENT_BINARY_DIR}/include")
option(THINKTHEN_ASAN "Instrument the C++ implementation for memory checks" OFF)
if(THINKTHEN_ASAN)
  target_compile_options(thinkthen_cpp_objects PRIVATE -fsanitize=address -fno-omit-frame-pointer)
endif()
add_library(thinkthen::native_shared SHARED IMPORTED GLOBAL)
set_target_properties(thinkthen::native_shared PROPERTIES IMPORTED_LOCATION "${THINKTHEN_NATIVE_SHARED}" IMPORTED_SONAME "libthinkthen.so.0")
add_library(thinkthen::native_static STATIC IMPORTED GLOBAL)
set_target_properties(thinkthen::native_static PROPERTIES IMPORTED_LOCATION "${THINKTHEN_NATIVE_STATIC}")
foreach(kind shared static)
  add_library(thinkthen_cpp_${kind} STATIC $<TARGET_OBJECTS:thinkthen_cpp_objects>)
  add_library(thinkthen::cpp_${kind} ALIAS thinkthen_cpp_${kind})
  target_compile_features(thinkthen_cpp_${kind} INTERFACE cxx_std_17)
  if(THINKTHEN_ASAN)
    target_link_options(thinkthen_cpp_${kind} INTERFACE -fsanitize=address)
  endif()
  target_include_directories(thinkthen_cpp_${kind} INTERFACE
    $<BUILD_INTERFACE:${CMAKE_CURRENT_SOURCE_DIR}/include>
    $<BUILD_INTERFACE:${CMAKE_CURRENT_BINARY_DIR}/include>
    $<INSTALL_INTERFACE:${CMAKE_INSTALL_INCLUDEDIR}>)
  if(kind STREQUAL "shared")
    target_link_libraries(thinkthen_cpp_${kind} INTERFACE thinkthen::native_shared)
  else()
    target_link_libraries(thinkthen_cpp_${kind} INTERFACE thinkthen::native_static pthread dl m)
  endif()
  install(TARGETS thinkthen_cpp_${kind} EXPORT ThinkThenCppTargets ARCHIVE DESTINATION ${CMAKE_INSTALL_LIBDIR})
endforeach()
file(MAKE_DIRECTORY "${CMAKE_CURRENT_BINARY_DIR}/include/thinkthen")
configure_file("${THINKTHEN_C_HEADER}" "${CMAKE_CURRENT_BINARY_DIR}/include/thinkthen/thinkthen.h" COPYONLY)
install(DIRECTORY include/ DESTINATION ${CMAKE_INSTALL_INCLUDEDIR} PATTERN native_implementation.hpp EXCLUDE)
install(FILES "${THINKTHEN_NATIVE_SHARED}" DESTINATION ${CMAKE_INSTALL_LIBDIR} RENAME libthinkthen.so.0)
install(FILES "${THINKTHEN_NATIVE_STATIC}" DESTINATION ${CMAKE_INSTALL_LIBDIR} RENAME libthinkthen.a)
configure_package_config_file(cmake/ThinkThenCppConfig.cmake.in
  "${CMAKE_CURRENT_BINARY_DIR}/thinkthen-cpp-config.cmake"
  INSTALL_DESTINATION ${CMAKE_INSTALL_LIBDIR}/cmake/thinkthen-cpp)
write_basic_package_version_file("${CMAKE_CURRENT_BINARY_DIR}/thinkthen-cpp-config-version.cmake"
  VERSION ${PROJECT_VERSION} COMPATIBILITY SameMajorVersion)
install(EXPORT ThinkThenCppTargets FILE thinkthen-cpp-targets.cmake
  NAMESPACE thinkthen:: DESTINATION ${CMAKE_INSTALL_LIBDIR}/cmake/thinkthen-cpp)
install(FILES "${CMAKE_CURRENT_BINARY_DIR}/thinkthen-cpp-config.cmake"
  "${CMAKE_CURRENT_BINARY_DIR}/thinkthen-cpp-config-version.cmake"
  DESTINATION ${CMAKE_INSTALL_LIBDIR}/cmake/thinkthen-cpp)

install(DIRECTORY examples/ DESTINATION ${CMAKE_INSTALL_DATADIR}/thinkthen-cpp/examples)
