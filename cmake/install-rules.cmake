install(
    TARGETS project_exe
    RUNTIME COMPONENT project_Runtime
)

if(PROJECT_IS_TOP_LEVEL)
  include(CPack)
endif()
