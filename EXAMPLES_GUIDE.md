# Rust Examples 使用指南

本指南详细说明如何在 `examples/` 目录下创建和运行示例项目。

## 目录

- [单文件示例](#单文件示例)
- [多文件示例](#多文件示例)
- [运行示例](#运行示例)
- [示例最佳实践](#示例最佳实践)
- [常见问题](#常见问题)

---

## 单文件示例

### 什么是单文件示例？

单文件示例是最简单的形式，整个示例代码都在一个 `.rs` 文件中。

### 创建单文件示例

```bash
# 在 chapter02/examples/ 目录下创建新文件
cat > chapter02/examples/my_example.rs << 'EOF'
fn main() {
    println!("=== 我的示例 ===");

    // 你的代码
    let result = 1 + 2;
    println!("结果: {}", result);
}
EOF
```

### 文件结构

```
chapter02/
├── examples/
│   ├── my_example.rs        ← 单文件示例
│   ├── another_example.rs   ← 另一个单文件示例
│   └── ...
└── src/
    └── main.rs
```

### 运行单文件示例

```bash
# 基本运行
cargo run -p chapter02 --example my_example

# Release 模式运行（更快）
cargo run -p chapter02 --example my_example --release

# 只编译不运行
cargo build -p chapter02 --example my_example
```

### 单文件示例的限制

- ✅ 简单直接
- ✅ 适合演示单个概念
- ❌ **不能包含其他模块文件**
- ❌ 所有代码必须在一个文件中

---

## 多文件示例

### 什么是多文件示例？

当你的示例需要多个模块文件时，需要创建一个子目录，并在其中创建 `main.rs` 和其他模块文件。

### 创建多文件示例

#### 步骤 1: 创建目录结构

```bash
# 创建示例目录
mkdir -p chapter02/examples/complex_example

# 目录结构
chapter02/examples/complex_example/
├── main.rs          ← 主入口文件（必须命名为 main.rs）
├── utils.rs         ← 工具模块
├── calculator.rs    ← 计算器模块
└── config.rs        ← 配置模块
```

#### 步骤 2: 创建 main.rs（入口文件）

```rust
// chapter02/examples/complex_example/main.rs

mod calculator;  // 声明模块
mod utils;
mod config;

use calculator::Calculator;

fn main() {
    println!("=== 复杂示例 ===");

    let calc = Calculator::new();
    let result = calc.add(10, 20);

    utils::print_result(result);
}
```

#### 步骤 3: 创建模块文件

**calculator.rs:**
```rust
// chapter02/examples/complex_example/calculator.rs

pub struct Calculator {
    name: String,
}

impl Calculator {
    pub fn new() -> Self {
        Calculator {
            name: String::from("Calculator"),
        }
    }

    pub fn add(&self, a: i32, b: i32) -> i32 {
        a + b
    }
}
```

**utils.rs:**
```rust
// chapter02/examples/complex_example/utils.rs

pub fn print_result(result: i32) {
    println!("结果: {}", result);
}

pub fn print_separator() {
    println!("{}", "-".repeat(40));
}
```

### 运行多文件示例

```bash
# 使用目录名作为示例名称
cargo run -p chapter02 --example complex_example

# Release 模式
cargo run -p chapter02 --example complex_example --release
```

### 完整示例结构

```
chapter02/
├── Cargo.toml
├── src/
│   └── main.rs
└── examples/
    ├── simple_example.rs           ← 单文件示例
    ├── another_simple.rs           ← 单文件示例
    └── complex_example/            ← 多文件示例
        ├── main.rs                 ← 必须是 main.rs
        ├── calculator.rs
        ├── utils.rs
        └── config.rs
```

---

## 运行示例

### 基本命令

```bash
# 运行单文件示例
cargo run -p <chapter> --example <example_name>

# 运行多文件示例
cargo run -p <chapter> --example <directory_name>
```

### 实际例子

```bash
# 运行 chapter01 的 variables 示例
cargo run -p chapter01 --example variables

# 运行 chapter02 的 float_types 示例
cargo run -p chapter02 --example float_types

# 运行 chapter02 的 complex_example 多文件示例
cargo run -p chapter02 --example complex_example
```

### 查看所有可用示例

```bash
# 方法 1: 列出文件
ls chapter02/examples/

# 方法 2: 使用 cargo
cargo build -p chapter02 --examples

# 方法 3: 查看帮助
cargo run -p chapter02 --example
```

### 编译但不运行

```bash
# 编译示例
cargo build -p chapter02 --example my_example

# 编译所有示例
cargo build -p chapter02 --examples

# Release 模式编译
cargo build -p chapter02 --example my_example --release
```

---

## 示例最佳实践

### 1. 命名规范

```bash
✅ 好的命名：
- variables.rs
- control_flow.rs
- error_handling.rs
- ownership_basics.rs

❌ 避免的命名：
- test.rs          # 太泛化
- example1.rs      # 不够描述性
- MyExample.rs     # 应使用 snake_case
```

### 2. 文件组织

**单文件示例（简单概念）：**
```
examples/
├── variables.rs
├── functions.rs
└── loops.rs
```

**多文件示例（复杂项目）：**
```
examples/
├── web_server/
│   ├── main.rs
│   ├── router.rs
│   ├── handler.rs
│   └── config.rs
└── database_example/
    ├── main.rs
    ├── models.rs
    └── queries.rs
```

### 3. 代码结构

每个示例应该：

```rust
fn main() {
    // 1. 打印标题
    println!("=== 示例名称 ===\n");

    // 2. 演示核心概念
    demonstrate_concept();

    // 3. 打印结果
    println!("\n=== 示例完成 ===");
}

fn demonstrate_concept() {
    // 你的示例代码
}
```

### 4. 添加注释

```rust
// ❌ 不好的注释
fn main() {
    let x = 5; // x 是 5
}

// ✅ 好的注释
fn main() {
    // 演示变量的不可变性
    // Rust 默认变量是不可变的
    let x = 5;
    println!("x = {}", x);

    // x = 6; // 错误！不能修改不可变变量
}
```

### 5. 错误处理

```rust
// ✅ 示例代码可以使用 unwrap()
fn main() {
    let file = std::fs::read_to_string("example.txt").unwrap();
    println!("{}", file);
}

// ✅ 但最好添加说明性注释
fn main() {
    // 注意：生产代码应该正确处理错误
    // 这里为了简化示例使用 unwrap()
    let file = std::fs::read_to_string("example.txt")
        .expect("无法读取文件");
    println!("{}", file);
}
```

---

## 常见问题

### Q1: 单文件示例和多文件示例如何选择？

**使用单文件示例当：**
- 代码少于 200 行
- 只演示一个概念
- 不需要多个模块

**使用多文件示例当：**
- 需要多个模块
- 演示完整的项目结构
- 代码超过 200 行
- 需要复用代码

### Q2: 为什么多文件示例必须使用 main.rs？

Cargo 的约定：
- 单文件示例：`examples/foo.rs` → `cargo run --example foo`
- 多文件示例：`examples/foo/main.rs` → `cargo run --example foo`

这是 Cargo 的内置规则，不能更改。

### Q3: 示例可以依赖外部 crate 吗？

可以！示例会自动使用包的所有依赖。

```rust
// chapter02/examples/using_serde.rs
use serde::{Serialize, Deserialize};  // 如果 chapter02 依赖了 serde

#[derive(Serialize, Deserialize)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let p = Point { x: 1, y: 2 };
    let json = serde_json::to_string(&p).unwrap();
    println!("{}", json);
}
```

### Q4: 示例可以使用测试吗？

可以，但通常不推荐在示例中使用测试。测试应该放在：
- `src/` 目录中的单元测试
- `tests/` 目录中的集成测试

如果确实需要：
```rust
// examples/my_example.rs
fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(2, 2), 4);
    }
}

fn main() {
    println!("2 + 2 = {}", add(2, 2));
}
```

### Q5: 如何在示例中使用本包的代码？

```rust
// chapter02/examples/using_lib.rs

// 如果 chapter02 有 lib.rs 或公共 API
use chapter02::some_function;

fn main() {
    some_function();
}
```

### Q6: 示例可以共享代码吗？

不能直接共享。每个示例都是独立的。

**解决方案：**

1. **使用 src/lib.rs 或公共库**
```rust
// 在 src/lib.rs 中定义
pub fn shared_function() {
    println!("这是共享函数");
}

// 在示例中使用
use chapter02::shared_function;
```

2. **使用 common 库**
```rust
use common::print_separator;
```

### Q7: 如何调试示例？

```bash
# 使用 println! 调试
cargo run -p chapter02 --example my_example

# 使用 dbg! 宏
cargo run -p chapter02 --example my_example

# 使用调试器（VS Code）
# 在 .vscode/launch.json 中配置
{
    "type": "lldb",
    "request": "launch",
    "name": "Debug example",
    "cargo": {
        "args": [
            "build",
            "-p", "chapter02",
            "--example=my_example"
        ]
    }
}
```

### Q8: 示例编译失败怎么办？

```bash
# 1. 清理并重新编译
cargo clean
cargo build -p chapter02 --example my_example

# 2. 查看详细错误
cargo build -p chapter02 --example my_example --verbose

# 3. 检查语法
cargo check -p chapter02 --example my_example

# 4. 使用 clippy 检查
cargo clippy -p chapter02 --example my_example
```

---

## 快速参考

### 创建单文件示例

```bash
# 1. 创建文件
touch chapter02/examples/my_example.rs

# 2. 编辑文件（添加 main 函数）

# 3. 运行
cargo run -p chapter02 --example my_example
```

### 创建多文件示例

```bash
# 1. 创建目录
mkdir -p chapter02/examples/my_complex_example

# 2. 创建 main.rs
touch chapter02/examples/my_complex_example/main.rs

# 3. 创建其他模块
touch chapter02/examples/my_complex_example/utils.rs

# 4. 在 main.rs 中声明模块
# mod utils;

# 5. 运行
cargo run -p chapter02 --example my_complex_example
```

### 常用命令总结

| 操作 | 命令 |
|------|------|
| 运行示例 | `cargo run -p <pkg> --example <name>` |
| 编译示例 | `cargo build -p <pkg> --example <name>` |
| 编译所有示例 | `cargo build -p <pkg> --examples` |
| Release 运行 | `cargo run -p <pkg> --example <name> --release` |
| 列出示例 | `ls <pkg>/examples/` |
| 检查示例 | `cargo check -p <pkg> --example <name>` |
| Clippy 检查 | `cargo clippy -p <pkg> --example <name>` |

---

## 示例模板

### 单文件示例模板

```rust
// chapter02/examples/template.rs
// 描述这个示例的目的

fn main() {
    println!("=== 示例标题 ===\n");

    // 第一部分：概念说明
    println!("1. 第一个概念:");
    demonstrate_first_concept();

    // 第二部分：进阶用法
    println!("\n2. 进阶用法:");
    demonstrate_advanced_usage();

    println!("\n=== 示例完成 ===");
}

fn demonstrate_first_concept() {
    // 实现代码
    println!("   演示内容...");
}

fn demonstrate_advanced_usage() {
    // 实现代码
    println!("   进阶内容...");
}
```

### 多文件示例模板

**main.rs:**
```rust
// chapter02/examples/template/main.rs

mod calculator;
mod utils;

use calculator::Calculator;

fn main() {
    println!("=== 多文件示例 ===\n");

    let calc = Calculator::new();
    let result = calc.calculate();

    utils::print_result(result);

    println!("\n=== 示例完成 ===");
}
```

**calculator.rs:**
```rust
// chapter02/examples/template/calculator.rs

pub struct Calculator;

impl Calculator {
    pub fn new() -> Self {
        Calculator
    }

    pub fn calculate(&self) -> i32 {
        42
    }
}
```

**utils.rs:**
```rust
// chapter02/examples/template/utils.rs

pub fn print_result(result: i32) {
    println!("结果: {}", result);
}
```

---

## 总结

- 📝 **单文件示例**：简单、直接，适合演示单个概念
- 📦 **多文件示例**：适合复杂项目，需要创建子目录和 `main.rs`
- 🚀 **运行命令**：`cargo run -p <package> --example <name>`
- ✅ **最佳实践**：清晰命名、充分注释、适当组织

Happy coding! 🦀
