# Rust From Zero - 学习项目

这是一个 Rust 学习书籍的示例代码项目，采用 Cargo Workspace 结构组织，每个章节都是独立的包，可以单独编译和运行。

## 项目结构

```
rust_from_0_project/
├── Cargo.toml              # Workspace 配置文件
├── README.md               # 本文件
├── common/                 # 公共库（所有章节共享的工具函数）
│   ├── Cargo.toml
│   └── src/
│       └── lib.rs
├── chapter01/              # 第一章：Rust 基础入门
│   ├── Cargo.toml
│   ├── src/
│   │   └── main.rs         # 章节主程序
│   └── examples/           # 本章的多个示例
│       ├── variables.rs
│       ├── functions.rs
│       └── control_flow.rs
├── chapter02/              # 第二章：所有权系统
│   ├── Cargo.toml
│   ├── src/
│   │   └── main.rs
│   └── examples/
│       ├── ownership.rs
│       ├── references.rs
│       └── slices.rs
└── chapter03/              # 第三章：进阶主题
    ├── Cargo.toml
    ├── src/
    │   └── main.rs
    └── examples/
        └── ...
```

## 快速开始

### 1. 编译整个项目

```bash
# 编译所有章节
cargo build

# 编译指定章节
cargo build -p chapter01
```

### 2. 运行章节主程序

```bash
# 运行第一章主程序
cargo run -p chapter01

# 运行第二章主程序
cargo run -p chapter02

# 运行第三章主程序
cargo run -p chapter03
```

### 3. 运行章节内的示例

每个章节的 `examples/` 目录下包含多个独立的示例程序：

```bash
# 运行 chapter01 的示例
cargo run -p chapter01 --example variables      # 变量和可变性
cargo run -p chapter01 --example functions      # 函数的使用
cargo run -p chapter01 --example control_flow   # 控制流

# 运行 chapter02 的示例
cargo run -p chapter02 --example ownership      # 所有权
cargo run -p chapter02 --example references     # 引用和借用
cargo run -p chapter02 --example slices         # 切片

# 运行 chapter03 的示例
cargo run -p chapter03 --example ownership      # 所有权深入
cargo run -p chapter03 --example borrowing      # 借用规则
cargo run -p chapter03 --example slices         # 切片详解
```

### 4. 列出所有可用的示例

```bash
# 查看某个章节的所有示例
cargo run -p chapter01 --example

# 或者直接查看 examples 目录
ls chapter01/examples/
```

### 5. 运行测试

```bash
# 运行所有测试
cargo test

# 运行指定章节的测试
cargo test -p chapter01
cargo test -p common
```

## 章节说明

### Chapter 01: Rust 基础入门

介绍 Rust 的基本概念：
- ✅ 变量和可变性
- ✅ 数据类型
- ✅ 函数
- ✅ 控制流（if、loop、while、for、match）
- ✅ 注释和文档

**示例程序：**
- `variables.rs` - 变量声明、可变性、遮蔽、常量
- `functions.rs` - 函数定义、参数、返回值、高阶函数
- `control_flow.rs` - 条件判断、循环、模式匹配

### Chapter 02: 所有权系统

深入理解 Rust 的核心特性：
- ✅ 所有权规则
- ✅ 移动和克隆
- ✅ 引用和借用
- ✅ 可变引用规则
- ✅ 切片类型

**示例程序：**
- `ownership.rs` - 所有权的转移和克隆（待创建）
- `references.rs` - 引用和借用的使用（待创建）
- `slices.rs` - 字符串切片和数组切片（待创建）

### Chapter 03: 进阶主题

更深入的 Rust 概念（待完善）

### Common 库

提供所有章节共享的工具函数：
- 格式化输出函数
- 数学工具（GCD、质数判断、斐波那契）
- 字符串处理工具
- 包含单元测试

## 如何添加新章节

### 方法 1: 手动创建

1. 创建章节目录和结构：
```bash
mkdir -p chapter04/src chapter04/examples
```

2. 创建章节的 `Cargo.toml`：
```toml
[package]
name = "chapter04"
version.workspace = true
edition.workspace = true

[dependencies]
# 使用 workspace 定义的依赖
# clap = { workspace = true }
# common = { path = "../common" }

[[bin]]
name = "chapter04"
path = "src/main.rs"
```

3. 创建 `src/main.rs` 和示例文件

4. 在根 `Cargo.toml` 的 `members` 数组中添加 `"chapter04"`

5. 构建新章节：
```bash
cargo build -p chapter04
```

### 方法 2: 使用 cargo 命令

```bash
# 在 workspace 根目录
cargo new chapter04 --name chapter04
mv chapter04/Cargo.toml chapter04/Cargo.toml.bak

# 然后手动编辑 Cargo.toml 使用 workspace 配置
```

## 开发建议

### 1. 使用公共库

在章节的 `Cargo.toml` 中添加依赖：
```toml
[dependencies]
common = { path = "../common" }
```

在代码中使用：
```rust
use common::print_chapter_header;

fn main() {
    print_chapter_header("Chapter 01", "Rust 基础");
}
```

### 2. 使用 Workspace 依赖

在根 `Cargo.toml` 的 `[workspace.dependencies]` 中定义公共依赖，各章节可以通过 `workspace = true` 引用：

```toml
# chapter01/Cargo.toml
[dependencies]
clap = { workspace = true }
```

### 3. 运行特定示例

```bash
# 只编译不运行
cargo build -p chapter01 --example variables

# 编译并运行
cargo run -p chapter01 --example variables

# 使用 release 模式（更快）
cargo run -p chapter01 --example variables --release
```

### 4. 清理构建

```bash
# 清理所有构建产物
cargo clean

# 只清理特定包
cargo clean -p chapter01
```

## 常见问题

### Q: 为什么使用 Workspace？

A: Workspace 允许我们：
- 每个章节独立管理依赖
- 共享 `target/` 目录，节省磁盘空间
- 可以单独编译和运行每个章节
- 清晰的项目组织结构

### Q: 如何在章节间共享代码？

A: 使用 `common` 库，或者创建其他共享库。

### Q: examples 和 bin 的区别？

A:
- `examples/` 用于演示代码，通过 `--example` 运行
- `src/bin/` 用于多个可执行程序，通过 `--bin` 运行
- 本项目使用 `examples/` 因为它更适合学习示例

### Q: 如何查看某个包的依赖树？

A:
```bash
cargo tree -p chapter01
```

## 学习路径建议

1. **从 Chapter 01 开始**，按顺序学习
2. **先运行主程序**，了解章节概览
3. **逐个运行示例**，深入理解每个概念
4. **阅读源代码**，理解实现细节
5. **修改代码实验**，加深理解
6. **自己编写新示例**，巩固知识

## 版本信息

- Rust Edition: 2021
- 最低 Rust 版本: 1.70+

## 贡献

欢迎提交 PR 添加新章节或改进现有示例！

## 许可证

MIT License
