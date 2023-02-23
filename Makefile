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
	-smp 4 -m 4G

ifeq ($(ARCH), x86_64)
QEMU_CMD += \
	-drive if=pflash,format=raw,readonly,file=$(OVMF) \
	-drive format=raw,file=fat:rw:$(ESP) \
	-device isa-debug-exit,iobase=0xf4,iosize=0x04 \
	-drive format=qcow2,file=disk.qcow2,media=disk,cache=writeback,id=sfsimg,if=none \
	-device ahci,id=ahci0 \
	-device ide-hd,drive=sfsimg,bus=ahci0.0 \
	-serial stdio -s -display none
endif

.PHONY: bootloader build qcow2 emu debug clean

bootloader:
ifeq ($(ARCH), x86_64)
	cd ${RBOOT_DIR} && make build
endif
	
build: bootloader
	cargo build $(CARGO_FLAGS)
	objdump --demangle -d ${KERNEL} > ${KERNEL}.asm
ifeq ($(ARCH), x86_64)
	mkdir -p $(ESP)/EFI/Demo $(ESP)/EFI/Boot
	cp ${RBOOT_DIR}/target/x86_64-unknown-uefi/$(PROFILE)/rboot.efi $(ESP)/EFI/Boot/BootX64.efi
	cp ${RBOOT_DIR}/rboot.conf $(ESP)/EFI/Boot/rboot.conf
	cp $(KERNEL) $(ESP)/EFI/Demo/kernel.elf
endif

qcow2:
ifeq ($(wildcard disk.qcow2),)
	dd if=/dev/zero of=disk.img bs=4096 count=92160 2>/dev/null
	mkfs.fat -F 32 disk.img
	qemu-img convert -f raw disk.img -O qcow2 disk.qcow2
	qemu-img resize disk.qcow2 +1G
endif

emu: build qcow2
	${QEMU_CMD}

debug: build
	${QEMU_CMD} -S

clean:
ifeq ($(ARCH), x86_64)
	cd ${RBOOT_DIR} && cargo clean
endif
	cargo clean