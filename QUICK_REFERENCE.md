# Rust From Zero - 快速参考指南

## 常用命令速查表

### 📦 编译和构建

```bash
# 编译整个工作空间
cargo build

# 编译指定章节
cargo build -p chapter01

# Release 模式编译（优化）
cargo build --release

# 检查代码（不生成可执行文件，更快）
cargo check
```

### 🚀 运行程序

```bash
# 运行章节主程序
cargo run -p chapter01
cargo run -p chapter02
cargo run -p chapter03

# 运行指定示例
cargo run -p chapter01 --example variables
cargo run -p chapter01 --example functions
cargo run -p chapter01 --example control_flow

# Release 模式运行（更快）
cargo run -p chapter01 --release
```

### 🧪 测试

```bash
# 运行所有测试
cargo test

# 运行指定包的测试
cargo test -p common
cargo test -p chapter01

# 运行指定测试（按名称过滤）
cargo test fibonacci

# 显示测试输出
cargo test -- --show-output
```

### 📋 查看信息

```bash
# 列出工作空间成员
cargo metadata --no-deps | grep '"name"'

# 查看依赖树
cargo tree
cargo tree -p chapter01

# 列出可用的示例（查看目录）
ls chapter01/examples/
ls chapter02/examples/

# 查看包信息
cargo metadata -p chapter01
```

### 🧹 清理

```bash
# 清理所有构建产物
cargo clean

# 清理指定包
cargo clean -p chapter01
```

## 章节运行命令快捷方式

### Chapter 01: Rust 基础入门

```bash
# 主程序
cargo run -p chapter01

# 示例程序
cargo run -p chapter01 --example variables       # 变量和可变性
cargo run -p chapter01 --example functions       # 函数详解
cargo run -p chapter01 --example control_flow    # 控制流
```

### Chapter 02: 所有权系统

```bash
# 主程序
cargo run -p chapter02

# 示例程序（待创建）
cargo run -p chapter02 --example ownership       # 所有权
cargo run -p chapter02 --example references      # 引用和借用
cargo run -p chapter02 --example slices          # 切片
```

### Chapter 03: 进阶主题

```bash
# 主程序
cargo run -p chapter03

# 示例程序（待创建）
cargo run -p chapter03 --example ownership       # 所有权深入
cargo run -p chapter03 --example borrowing       # 借用规则
cargo run -p chapter03 --example slices          # 切片详解
```

## 项目结构速览

```
rust_from_0_project/
├── Cargo.toml              ← Workspace 配置
├── README.md               ← 详细文档
├── QUICK_REFERENCE.md      ← 本文件
│
├── common/                 ← 公共工具库
│   ├── Cargo.toml
│   └── src/lib.rs          ← 共享函数
│
├── chapter01/              ← 第一章
│   ├── Cargo.toml
│   ├── src/main.rs         ← 章节主程序
│   └── examples/           ← 示例程序
│       ├── variables.rs
│       ├── functions.rs
│       └── control_flow.rs
│
├── chapter02/              ← 第二章
│   └── ...
│
└── chapter03/              ← 第三章
    └── ...
```

## 添加新章节步骤

### 方法 1: 快速创建（推荐）

```bash
# 1. 创建目录结构
mkdir -p chapter04/src chapter04/examples

# 2. 创建 Cargo.toml
cat > chapter04/Cargo.toml << 'EOF'
[package]
name = "chapter04"
version.workspace = true
edition.workspace = true

[dependencies]
# common = { path = "../common" }

[[bin]]
name = "chapter04"
path = "src/main.rs"
EOF

# 3. 创建主程序
cat > chapter04/src/main.rs << 'EOF'
fn main() {
    println!("=== Chapter 04 ===");
}
EOF

# 4. 编辑根 Cargo.toml，在 members 中添加 "chapter04"
# 手动编辑或使用 sed/awk

# 5. 测试构建
cargo build -p chapter04
cargo run -p chapter04
```

### 方法 2: 复制现有章节

```bash
# 复制章节
cp -r chapter01 chapter04

# 修改 chapter04/Cargo.toml 中的 name = "chapter04"
# 修改 chapter04/src/main.rs 的内容
# 在根 Cargo.toml 的 members 中添加 "chapter04"
```

## 添加新示例步骤

```bash
# 1. 创建示例文件
cat > chapter01/examples/new_example.rs << 'EOF'
fn main() {
    println!("=== 新示例 ===");
}
EOF

# 2. 运行示例
cargo run -p chapter01 --example new_example
```

## 使用公共库

### 在 Cargo.toml 中添加依赖

```toml
[dependencies]
common = { path = "../common" }
```

### 在代码中使用

```rust
use common::print_chapter_header;
use common::fibonacci;

fn main() {
    print_chapter_header("Chapter 01", "Rust 基础");

    for i in 0..10 {
        println!("{}", fibonacci(i));
    }
}
```

## 常见问题快速解决

### Q: 找不到包 (package not found)

```bash
# 确保包在 workspace members 中
cat Cargo.toml | grep members -A 10

# 重新构建
cargo clean
cargo build
```

### Q: 示例找不到 (example not found)

```bash
# 检查文件是否存在
ls chapter01/examples/

# 确保文件名正确（不含 .rs 后缀）
cargo run -p chapter01 --example variables  # ✅ 正确
cargo run -p chapter01 --example variables.rs  # ❌ 错误
```

### Q: 依赖版本冲突

```bash
# 更新依赖
cargo update

# 查看依赖树
cargo tree -p chapter01
```

### Q: 编译错误

```bash
# 清理后重新编译
cargo clean
cargo build

# 检查 Rust 版本
rustc --version
rustup update
```

## 性能优化

```bash
# Release 模式（优化编译）
cargo build --release
cargo run -p chapter01 --release

# 查看二进制文件大小
ls -lh target/release/chapter01

# 使用 LTO（链接时优化）
# 在 Cargo.toml 添加：
# [profile.release]
# lto = true
```

## 代码格式化和检查

```bash
# 格式化代码
cargo fmt

# 检查代码风格
cargo clippy

# 检查所有警告
cargo clippy -- -W clippy::all
```

## 文档

```bash
# 生成文档
cargo doc

# 生成并打开文档
cargo doc --open

# 为指定包生成文档
cargo doc -p common --open
```

## 快捷别名（可选）

在 `~/.bashrc` 或 `~/.zshrc` 中添加：

```bash
# Rust From Zero 项目别名
alias rfz='cd /path/to/rust_from_0_project'
alias c1='cargo run -p chapter01'
alias c2='cargo run -p chapter02'
alias c3='cargo run -p chapter03'
alias cb='cargo build'
alias ct='cargo test'
alias cc='cargo clean'

# 示例别名
alias c1-vars='cargo run -p chapter01 --example variables'
alias c1-funcs='cargo run -p chapter01 --example functions'
alias c1-ctrl='cargo run -p chapter01 --example control_flow'
```

## Git 工作流（如果使用版本控制）

```bash
# 初始化仓库
git init

# .gitignore 已配置好（忽略 target/ 等）

# 提交代码
git add .
git commit -m "Add chapter04"

# 为每个章节打标签
git tag -a chapter01-complete -m "完成第一章"
git tag -a chapter02-complete -m "完成第二章"
```

## 学习建议

1. **按顺序学习** - 从 chapter01 开始
2. **先读后跑** - 先阅读代码，再运行看效果
3. **动手修改** - 修改代码，观察变化
4. **自己实现** - 尝试不看代码自己实现
5. **写笔记** - 记录重要概念和易错点

## 资源链接

- [The Rust Book](https://doc.rust-lang.org/book/)
- [Rust By Example](https://doc.rust-lang.org/rust-by-example/)
- [Cargo Book](https://doc.rust-lang.org/cargo/)
- [Rust Playground](https://play.rust-lang.org/)

---

**提示**: 将此文件加入书签，随时查阅常用命令！
