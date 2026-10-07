# Both build paths compile the same SQL adapter.
set(THINKTHEN_CPP_SOURCES)
foreach(SOURCE thinkthen find scalar_owner scalar_settings portable plan listed_result
    nested nested_result complete complete_files usage removed relate relate_query files files_manifest images)
  list(APPEND THINKTHEN_CPP_SOURCES "${CMAKE_CURRENT_LIST_DIR}/src/${SOURCE}.cpp")
endforeach()
