# https://github.com/casey/just

NVCC_FLAGS := "-arch=sm_61 -Wno-deprecated-gpu-targets -w -O3"

link: compile
    cd cuda/target && nvcc {{NVCC_FLAGS}} hvm2.obj -o hvm2

compile: target-dir
    cd cuda/target && nvcc {{NVCC_FLAGS}} -c ../hvm2.cu -o hvm2.obj

lib: compile
    cd cuda/target && lib /out:hvm2.lib hvm2.obj

ptx: target-dir
    cd cuda/target && nvcc {{NVCC_FLAGS}} -ptx ../hvm2.cu -o hvm2.ptx

target-dir:
    mkdir -p cuda/target
