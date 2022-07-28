# @Author: strluck
# @Date: 2022-07-28 01:07:14
# @Last Modified time: 2022-07-28 01:09:00

BIN_DIR = ./bin
TARGET_DIR = ./scripts/target



.PHONY: run
# 构建并运行
run:
	@make build
	@godot -d

.PHONY: build
# 构建
build:
	@echo start build...
	@cd scripts && cargo build
	@make init-python-cp
	@python $(TARGET_DIR)/cp.py $(TARGET_DIR)/debug/scripts.dll $(BIN_DIR)/windows/scripts.dll
	@echo finish build ...


.PHONY: run-x86_64-pc-windows-gnu-debug
# 构建并运行 x86_64-pc-windows-gnu
run-x86_64-pc-windows-gnu-debug:
	@make build-x86_64-pc-windows-gnu-debug
	@godot -d

.PHONY: build-x86_64-pc-windows-gnu-debug
TARGET = "x86_64-pc-windows-gnu"
# 构建 x86_64-pc-windows-gnu
build-x86_64-pc-windows-gnu-debug:
	@echo start build $(TARGET)...
	@cd scripts && cargo build --target $(TARGET)
	@make init-python-cp
	@python $(TARGET_DIR)/cp.py $(TARGET_DIR)/$(TARGET)/debug/scripts.dll $(BIN_DIR)/windows/scripts.dll
	@echo finish build $(TARGET)...



########## .py ##########

.PHONY: init-python-cp
# 生成拷贝文件的python脚本, 用于兼容全平台
init-python-cp:
	@echo import sys> $(TARGET_DIR)/cp.py
	@echo import shutil>> $(TARGET_DIR)/cp.py
	@echo source, target = sys.argv[1], sys.argv[2]>> $(TARGET_DIR)/cp.py
	@echo shutil.copy(source, target)>> $(TARGET_DIR)/cp.py
