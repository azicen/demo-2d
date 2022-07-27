# @Author: strluck
# @Date: 2022-07-28 01:07:14
# @Last Modified time: 2022-07-28 01:09:00

.PHONY: build
# 构建
build:
	@echo start build...
	@cd scripts && cargo build

.PHONY: build-x86_64-pc-windows-gnu-debug
TARGET = "x86_64-pc-windows-gnu"
# 构建并运行debug x86_64-pc-windows-gnu
build-x86_64-pc-windows-gnu-debug:
	@echo start build $(TARGET)...
	@cd scripts && cargo build --target $(TARGET)
	@make init-python-cp
	@python scripts/target/cp.py ./scripts/target/$(TARGET)/debug/scripts.dll ./scripts/gdnative/bin/windows/scripts.dll
	@echo finish build $(TARGET)...



########## .py ##########

.PHONY: init-python-cp
# 生成拷贝文件的python脚本, 用于兼容全平台
init-python-cp:
	@echo import sys> scripts/target/cp.py
	@echo import shutil>> scripts/target/cp.py
	@echo source, target = sys.argv[1], sys.argv[2]>> scripts/target/cp.py
	@echo shutil.copy(source, target)>> scripts/target/cp.py
