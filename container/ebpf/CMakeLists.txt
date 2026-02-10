# eBPF kernel-side programs
# These require clang with -target bpf and special compilation flags.
# Uncomment and configure when eBPF programs are added.

# find_program(CLANG clang REQUIRED)
#
# set(EBPF_C_FLAGS -target bpf -O2 -g -Wall)
#
# function(add_ebpf_program name source)
#     add_custom_command(
#         OUTPUT ${CMAKE_CURRENT_BINARY_DIR}/${name}.bpf.o
#         COMMAND ${CLANG} ${EBPF_C_FLAGS}
#             -c ${CMAKE_CURRENT_SOURCE_DIR}/${source}
#             -o ${CMAKE_CURRENT_BINARY_DIR}/${name}.bpf.o
#         DEPENDS ${CMAKE_CURRENT_SOURCE_DIR}/${source}
#     )
#     add_custom_target(${name} ALL DEPENDS ${CMAKE_CURRENT_BINARY_DIR}/${name}.bpf.o)
# endfunction()
