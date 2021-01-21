cmake_minimum_required(VERSION 2.8 FATAL_ERROR)
project(cqdb)
enable_language(C)

include_directories(${PROJECT_SOURCE_DIR}/include)
set(CQDB_SOURCES
    ${PROJECT_SOURCE_DIR}/src/lookup3.c
    ${PROJECT_SOURCE_DIR}/src/cqdb.c)
add_library(cqdb STATIC ${CQDB_SOURCES})
set_target_properties(cqdb PROPERTIES PUBLIC_HEADER "include/cqdb.h")
install(TARGETS cqdb ARCHIVE DESTINATION lib PUBLIC_HEADER DESTINATION include)

include(CMakePackageConfigHelpers)
configure_package_config_file(
	"${PROJECT_SOURCE_DIR}/Config.cmake.in"
	"${PROJECT_BINARY_DIR}/${PROJECT_NAME}Config.cmake"
	INSTALL_DESTINATION "lib/cqdb/cmake"
	NO_SET_AND_CHECK_MACRO
	NO_CHECK_REQUIRED_COMPONENTS_MACRO
)
install(FILES "${PROJECT_BINARY_DIR}/${PROJECT_NAME}Config.cmake" DESTINATION "lib/cqdb/cmake" COMPONENT Development)
