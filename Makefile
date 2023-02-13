ARCH ?= x86_64
MACHINE ?= qemu
PROFILE ?= debug

build: 
	cargo build -Z build-std=core,alloc
	objcopy -O elf32-i386 target/${ARCH}${MACHINE}/$(PROFILE)/x86_demo

emu: build
	qemu-system-x86_64 \
    	-cpu qemu64,apic,fsgsbase,fxsr,rdrand,rdtscp,xsave,xsaveopt \
    	-smp 1 -m 64M -display none -serial stdio \
    	-kernel target/${ARCH}${MACHINE}/$(PROFILE)/x86_demo -s

debug: build
	qemu-system-x86_64 \
    	-cpu qemu64,apic,fsgsbase,fxsr,rdrand,rdtscp,xsave,xsaveopt \
    	-smp 1 -m 64M -display none -serial stdio \
    	-kernel target/${ARCH}${MACHINE}/$(PROFILE)/x86_demo -s -S

clean:
	-cargo clean