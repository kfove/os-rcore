TARGET := riscv64gc-unknown-none-elf
MODE := release
KERNEL_ELF := target/$(TARGET)/$(MODE)/os
KERNEL_BIN := target/$(TARGET)/$(MODE)/os.bin
DISASM_TMP := taget/$(TARGET)/$(MODE)/asm

# 构建模式 Debug / Release 这里默认 release
ifeq (${MODE}, release)
	MODE_ARG := --release
endif

# Board 板子
# 记录平台信息
BOARD ?= qemu
SBI ?= rustsbi
BOOTLOADER := ../bootloader/rustsbi-qemu.bin
k210_BOOTLOADER_SIZE := 131072	

# 内核入口
# 不同板子或者平台可能入口不同
ifeq (${BOARD}, qemu)
	KERNEL_ENTRY_PA := 0x80200000
else ifeq (${BOARD}, k210)
	KERNEL_ENTRY_PA := 0x80020000
endif

# 我手上没有 k210


# Bunutils 一套编译程序， 这里用途是导出镜像
OBJDUMP := rust-objdump --arch-name=riscv64
OBJCOPY := rust-objcopy --binary-architecture=riscv64

# 忽略ASM DISASM
DISASM ?= -x

# 构建时确认环境, 依次执行 env switch-check
build: ${KERNEL_BIN}

switch-check:
ifeq (${BOARD}, qemu)
	(which last-qemu) || (rm -f last-k210 && touch last-qemu && make clean)
endif

# 环境配置
env:
	(rustup target list | grep "riscv64gc-unknown-none-elf (installed)") || rustup target add $(TARGET)
	cargo install cargo-binutils
	rustup component add rust-src
	rustup component add llvm-tools-preview
# 这里导出镜像
${KERNEL_BIN}: kernel
	@$(OBJCOPY) $(KERNEL_ELF) --strip-all -O binary target/${TARGET}/${MODE}/os.bin

# 手上没有 k210 
kernel:
	@echo Platform: $(BOARD)
	@cargo build $(MODE_ARG)

# 清除构建
clean:
	@cargo clean

disasm: kernel
	@$(OBJDUMP) $(DISASM) $(KERNEL_ELF) | less

disasm-vim: kernel
	@$(OBJDUMP) $(DISASM) $(KERNEL_ELF) > $(DISASM_TMP)
	@vim $(DISASM_TMP)
	@rm $(DISASM_TMP)

# 这里就是运行器, 用于运行 qemu
run: run-inner


# 具体运行
run-inner: build
ifeq (${BOARD}, qemu)
	@echo "file $(KERNEL_BIN)"
	@echo "addr ${KERNEL_ENTRY_PA}"
	@echo "bootloader ${BOOTLOADER}"
	@qemu-system-riscv64 \
		-machine virt \
		-nographic \
		-bios $(BOOTLOADER) \
		-device loader,file=$(KERNEL_BIN),addr=$(KERNEL_ENTRY_PA) \
		-s -S
endif

gdb:
	@echo "file ${KERNEL_BIN}"
	@riscv64-elf-gdb \
		-ex "file ${KERNEL_BIN}" \
		-ex "target remote :1234"
