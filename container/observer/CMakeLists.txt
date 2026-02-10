add_executable(observer
    src/ebpf_loader.cpp
    src/event_reader.cpp
)

target_include_directories(observer PRIVATE
    ${CMAKE_CURRENT_SOURCE_DIR}/include
)

target_link_libraries(observer PRIVATE common)
