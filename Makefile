ARCH ?= x86_64
MACHINE ?= qemu
PROFILE ?= release

export RUSTFLAGS := ${RUSTFLAGS} -C force-frame-pointers=yes

# Cargo flags.
ifeq (${PROFILE}, release)
CARGO_FLAGS = --release  --no-default-features
else
CARGO_FLAGS =  --no-default-features
endif

# CARGO_FLAGS := ${CARGO_FLAGS} --features "${MACHINE}"

RBOOT_DIR := ./rboot
KERNEL_DIR := target/$(ARCH)${MACHINE}/$(PROFILE)
KERNEL := ${KERNEL_DIR}/x86_demo
ESP := $(KERNEL_DIR)/esp

OVMF := ${RBOOT_DIR}/OVMF.fd

QEMU := qemu-system-$(ARCH)

QEMU_CMD := ${QEMU} \
	-cpu qemu64,apic,fsgsbase,fxsr,rdrand,rdtscp,xsave,xsaveopt \
	-smp 1 -m 4G

ifeq ($(ARCH), x86_64)
QEMU_CMD += \
	-drive if=pflash,format=raw,readonly,file=$(OVMF) \
	-drive format=raw,file=fat:rw:$(ESP) \
	-device isa-debug-exit,iobase=0xf4,iosize=0x04 \
	-serial stdio -s -display none
endif

${KERNEL}:
	cargo build $(CARGO_FLAGS)

build: ${KERNEL}
	# cargo bootimage
	objdump --demangle -d ${KERNEL} > ${KERNEL}.asm
ifeq ($(ARCH), x86_64)
	cd ${RBOOT_DIR} && make build
	mkdir -p $(ESP)/EFI/Demo $(ESP)/EFI/Boot
	cp ${RBOOT_DIR}/target/x86_64-unknown-uefi/$(PROFILE)/rboot.efi $(ESP)/EFI/Boot/BootX64.efi
	cp ${RBOOT_DIR}/rboot.conf $(ESP)/EFI/Boot/rboot.conf
	cp $(KERNEL) $(ESP)/EFI/Demo/kernel.elf
endif

emu: build
	${QEMU_CMD}
	# qemu-system-x86_64 \
	# 	-cpu qemu64,apic,fsgsbase,fxsr,rdrand,rdtscp,xsave,xsaveopt \
	# 	-drive format=raw,file=target/x86_64qemu/debug/bootimage-x86_demo.bin \
	# 	-device isa-debug-exit,iobase=0xf4,iosize=0x04 \
	# 	-smp 1 -m 64M -display none -serial stdio -s

debug: build
	qemu-system-x86_64 \
		-cpu qemu64,apic,fsgsbase,fxsr,rdrand,rdtscp,xsave,xsaveopt \
		-drive format=raw,file=target/x86_64qemu/debug/bootimage-x86_demo.bin \
		-smp 1 -m 64M -display none -serial stdio -s -S

bootloader: $(kernel)
ifeq ($(ARCH), x86_64)
	@cd rboot && make build
endif

clean:
	-cargo clean