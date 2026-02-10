add_executable(sandbox
    src/main.cpp
    src/sandbox.cpp
    src/isolation/CapabilityDropper.cpp
    src/isolation/FileSystem.cpp
    src/isolation/NamespaceHandler.cpp
    src/isolation/SeccompProfile.cpp
    src/resources/CgroupManager.cpp
    src/resources/NetworkManager.cpp
    src/resources/ResourceManager.cpp
)

target_include_directories(sandbox PRIVATE
    ${CMAKE_CURRENT_SOURCE_DIR}/include
)

target_link_libraries(sandbox PRIVATE common)
