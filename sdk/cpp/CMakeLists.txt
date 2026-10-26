cmake_minimum_required(VERSION 3.20)
file(STRINGS "${CMAKE_CURRENT_SOURCE_DIR}/../../Cargo.toml" version_line REGEX "^version = ")
string(REGEX REPLACE "^version = \"([^\"]+)\"$" "\\1" package_version "${version_line}")
project(ThinkThenCpp VERSION "${package_version}" LANGUAGES CXX)
include(GNUInstallDirs)
include(CMakePackageConfigHelpers)
find_package(ThinkThenC ${PROJECT_VERSION} CONFIG REQUIRED)
add_library(thinkthen_cpp INTERFACE)
add_library(ThinkThen::Cpp ALIAS thinkthen_cpp)
set_target_properties(thinkthen_cpp PROPERTIES EXPORT_NAME Cpp)
target_compile_features(thinkthen_cpp INTERFACE cxx_std_17)
target_include_directories(thinkthen_cpp INTERFACE $<INSTALL_INTERFACE:${CMAKE_INSTALL_INCLUDEDIR}>)
target_link_libraries(thinkthen_cpp INTERFACE ThinkThen::C)
install(TARGETS thinkthen_cpp EXPORT ThinkThenCppTargets)
install(DIRECTORY include/ DESTINATION ${CMAKE_INSTALL_INCLUDEDIR})
install(FILES ../../LICENSE README.md DESTINATION ${CMAKE_INSTALL_DATADIR}/thinkthen-cpp)
configure_package_config_file(cmake/ThinkThenCppConfig.cmake.in
    "${CMAKE_CURRENT_BINARY_DIR}/ThinkThenCppConfig.cmake"
    INSTALL_DESTINATION ${CMAKE_INSTALL_LIBDIR}/cmake/ThinkThenCpp)
write_basic_package_version_file("${CMAKE_CURRENT_BINARY_DIR}/ThinkThenCppConfigVersion.cmake"
    VERSION "${PROJECT_VERSION}" COMPATIBILITY SameMinorVersion)
install(EXPORT ThinkThenCppTargets NAMESPACE ThinkThen:: DESTINATION ${CMAKE_INSTALL_LIBDIR}/cmake/ThinkThenCpp)
install(FILES "${CMAKE_CURRENT_BINARY_DIR}/ThinkThenCppConfig.cmake"
    "${CMAKE_CURRENT_BINARY_DIR}/ThinkThenCppConfigVersion.cmake" DESTINATION ${CMAKE_INSTALL_LIBDIR}/cmake/ThinkThenCpp)
