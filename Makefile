ARCH ?= x86_64
MACHINE ?= qemu
PROFILE ?= debug

build: 
	cargo build --lib --target ${ARCH}${MACHINE}.json -Z build-std=core,alloc

clean:
	-cargo clean