ARCH ?= x86_64
MACHINE ?= qemu
PROFILE ?= debug

build: 
	cargo bootimage

emu: build
	qemu-system-x86_64 \
		-cpu qemu64,apic,fsgsbase,fxsr,rdrand,rdtscp,xsave,xsaveopt \
		-drive format=raw,file=target/x86_64qemu/debug/bootimage-x86_demo.bin \
		-device isa-debug-exit,iobase=0xf4,iosize=0x04 \
		-smp 1 -m 64M -display none -serial stdio -s

debug: build
	qemu-system-x86_64 \
		-cpu qemu64,apic,fsgsbase,fxsr,rdrand,rdtscp,xsave,xsaveopt \
		-drive format=raw,file=target/x86_64qemu/debug/bootimage-x86_demo.bin \
		-smp 1 -m 64M -display none -serial stdio -s -S

clean:
	-cargo clean