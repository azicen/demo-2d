# @Author: strluck
# @Date: 2022-07-28 01:07:14
# @Last Modified time: 2022-07-28 01:09:00

ifeq ($(OS), Windows_NT)
	CP = powershell cp
else
	CP = bash cp
endif

PROJECT_NAME = elaiki_core

BIN_DIR = ./bin
TARGET_DIR = ./elaiki_core/target


.PHONY: init
# 初始化环境
init:
	@rustup toolchain install stable-x86_64-pc-windows-gnu
	@rustup target add x86_64-pc-windows-gnu


.PHONY: run
# 构建并运行
run:
	@make build
	@godot -d

.PHONY: build
# 构建
build:
	@echo start build...
	@cd $(PROJECT_NAME) && cargo build
	@$(CP) $(TARGET_DIR)/debug/$(PROJECT_NAME).dll $(BIN_DIR)/windows/scripts.dll
	@echo finish build ...


.PHONY: run-x86_64-pc-windows-gnu-debug
# 构建并运行 x86_64-pc-windows-gnu
run-x86_64-pc-windows-gnu-debug:
	@make build-x86_64-pc-windows-gnu-debug
	@godot -d

.PHONY: build-x86_64-pc-windows-gnu-debug
TARGET = x86_64-pc-windows-gnu
# 构建 x86_64-pc-windows-gnu
build-x86_64-pc-windows-gnu-debug:
	echo $(SHELL)
	@echo start build $(TARGET)...
	@cd $(PROJECT_NAME) && cargo build --target $(TARGET)
	@$(CP) $(TARGET_DIR)/$(TARGET)/debug/$(PROJECT_NAME).dll $(BIN_DIR)/windows/scripts.dll
	@echo finish build $(TARGET)...


.PHONY: run-x86_64-pc-windows-msvc-debug
# 构建并运行 x86_64-pc-windows-msvc
run-x86_64-pc-windows-msvc-debug:
	@make build-x86_64-pc-windows-msvc-debug
	@godot -d

.PHONY: build-x86_64-pc-windows-msvc-debug
TARGET = x86_64-pc-windows-msvc
# 构建 x86_64-pc-windows-msvc
build-x86_64-pc-windows-msvc-debug:
	echo $(SHELL)
	@echo start build $(TARGET)...
	@cd $(PROJECT_NAME) && cargo build --target $(TARGET)
	@$(CP) $(TARGET_DIR)/$(TARGET)/debug/$(PROJECT_NAME).dll $(BIN_DIR)/windows/scripts.dll
	@echo finish build $(TARGET)...
