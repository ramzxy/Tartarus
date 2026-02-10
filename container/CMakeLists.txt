cmake_minimum_required(VERSION 3.20)
project(Hamistegan LANGUAGES CXX)

set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_EXPORT_COMPILE_COMMANDS ON)

add_subdirectory(common)
add_subdirectory(sandbox)
add_subdirectory(observer)
# add_subdirectory(ebpf)  # uncomment when eBPF programs are added
