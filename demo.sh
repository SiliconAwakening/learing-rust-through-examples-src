#!/bin/bash

# Rust From Zero - 快速演示脚本
# 用于快速展示项目的各个示例

set -e

# 颜色定义
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
PURPLE='\033[0;35m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

# 打印带颜色的标题
print_title() {
    echo -e "\n${CYAN}========================================${NC}"
    echo -e "${CYAN}  $1${NC}"
    echo -e "${CYAN}========================================${NC}\n"
}

# 打印章节标题
print_chapter() {
    echo -e "\n${PURPLE}>>> $1${NC}\n"
}

# 打印成功信息
print_success() {
    echo -e "${GREEN}✓ $1${NC}"
}

# 打印信息
print_info() {
    echo -e "${BLUE}ℹ $1${NC}"
}

# 打印警告
print_warning() {
    echo -e "${YELLOW}⚠ $1${NC}"
}

# 等待用户按键
wait_for_key() {
    echo -e "\n${YELLOW}按 Enter 继续，或 Ctrl+C 退出...${NC}"
    read -r
}

# 运行命令并显示
run_command() {
    echo -e "${GREEN}$ $1${NC}"
    eval "$1"
    echo ""
}

# 清屏
clear

print_title "Rust From Zero - 项目演示"

echo -e "${BLUE}这个演示将展示项目的各个章节和示例${NC}"
echo -e "${BLUE}你可以随时按 Ctrl+C 退出${NC}"

wait_for_key

# 1. 显示项目结构
print_chapter "1. 项目结构"
print_info "显示项目目录结构..."
if command -v tree &> /dev/null; then
    tree -I target -L 2 --dirsfirst
else
    find . -not -path "*/target/*" -not -path "*/.git/*" -type d | head -20 | sort
fi

wait_for_key

# 2. 编译整个项目
print_chapter "2. 编译整个工作空间"
run_command "cargo build"
print_success "项目编译成功！"

wait_for_key

# 3. 运行 Chapter 01
print_chapter "3. 运行 Chapter 01: Rust 基础入门"
print_info "运行章节主程序..."
run_command "cargo run -p chapter01"

wait_for_key

# 4. 运行 Chapter 01 示例
print_chapter "4. Chapter 01 示例演示"

print_info "示例 1: 变量和可变性"
run_command "cargo run -p chapter01 --example variables"

wait_for_key

print_info "示例 2: 函数的使用"
run_command "cargo run -p chapter01 --example functions"

wait_for_key

print_info "示例 3: 控制流"
run_command "cargo run -p chapter01 --example control_flow"

wait_for_key

# 5. 运行 Chapter 02
print_chapter "5. 运行 Chapter 02: 所有权系统"
run_command "cargo run -p chapter02"

wait_for_key

# 6. 运行 Chapter 03
print_chapter "6. 运行 Chapter 03: 进阶主题"
run_command "cargo run -p chapter03"

wait_for_key

# 7. 运行测试
print_chapter "7. 运行测试套件"
print_info "运行公共库的单元测试..."
run_command "cargo test -p common"
print_success "所有测试通过！"

wait_for_key

# 8. 显示帮助信息
print_chapter "8. 常用命令"

echo -e "${BLUE}编译和运行:${NC}"
echo "  cargo build                          # 编译所有章节"
echo "  cargo run -p chapter01               # 运行第一章"
echo "  cargo run -p chapter01 --example variables  # 运行示例"
echo ""

echo -e "${BLUE}测试:${NC}"
echo "  cargo test                           # 运行所有测试"
echo "  cargo test -p common                 # 运行特定包的测试"
echo ""

echo -e "${BLUE}清理:${NC}"
echo "  cargo clean                          # 清理构建产物"
echo ""

echo -e "${BLUE}查看信息:${NC}"
echo "  cargo tree                           # 查看依赖树"
echo "  ls chapter01/examples/               # 列出示例文件"
echo ""

wait_for_key

# 9. 完成
print_title "演示完成！"

echo -e "${GREEN}✓ 你已经了解了项目的基本结构和使用方法${NC}"
echo ""
echo -e "${BLUE}下一步建议:${NC}"
echo "  1. 阅读 README.md 了解详细文档"
echo "  2. 查看 QUICK_REFERENCE.md 获取快速参考"
echo "  3. 按顺序学习各个章节的代码"
echo "  4. 尝试修改代码并运行"
echo "  5. 编写自己的示例代码"
echo ""
echo -e "${YELLOW}快速开始:${NC}"
echo "  cargo run -p chapter01 --example variables"
echo ""
echo -e "${CYAN}祝你学习愉快！🚀${NC}\n"
