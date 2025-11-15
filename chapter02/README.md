# Chapter 02 - 表达式计算器

一个功能完整的数学表达式求值器，支持基本运算、函数调用、括号以及 Unicode 数学符号。

## 功能特性 ✨

### 核心功能

- ✅ **完整的表达式求值系统**
  - 词法分析（Tokenization）
  - 语法分析（Parsing with AST）
  - 求值（Evaluation）

- ✅ **运算符支持**
  - 基本运算：`+`, `-`, `*`, `/`
  - 幂运算：`^` (右结合)
  - 一元运算符：`+`, `-`
  - Unicode 运算符：`×`, `÷`, `²`, `³`, `√`

- ✅ **数学函数**
  - 三角函数：`sin`, `cos`, `tan`
  - 平方根：`sqrt` 或 `√`
  - 对数：`ln`, `log`
  - 其他：`abs`, `ceil`, `floor`, `round`

- ✅ **常量**
  - 圆周率：`π` (pi ≈ 3.14159...)

- ✅ **正确的运算符优先级**
  1. 括号 `()`
  2. 函数调用和前缀运算符
  3. 一元运算符
  4. 幂运算（右结合）
  5. 乘除
  6. 加减

## 快速开始 🚀

### 运行交互式计算器

```bash
cargo run -p chapter02 --example expression_calculator
```

### 作为库使用

```rust
use chapter02::Calculator;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let calc = Calculator::new();

    // ASCII 语法
    let result = calc.evaluate_expression("2 + 3 * 4")?;
    println!("2 + 3 * 4 = {}", result); // 14.0

    // Unicode 语法（更简洁！）
    let result = calc.evaluate_expression("√16 + π × 5²")?;
    println!("√16 + π × 5² = {}", result); // 82.539...

    Ok(())
}
```

## 表达式示例 📝

### 基本运算

```
2 + 3                → 5
10 - 4 * 2           → 2
(10 - 4) * 2         → 12
2 ^ 3                → 8
2 ^ 3 ^ 2            → 512 (右结合)
```

### Unicode 符号

```
√16                  → 4
5²                   → 25
3³                   → 27
π                    → 3.141592653589793
5 × 3                → 15
20 ÷ 4               → 5
```

### 函数调用

```
sqrt(16)             → 4
sin(0)               → 0
abs(-42)             → 42
ceil(3.14)           → 4
floor(3.99)          → 3
```

### 复杂表达式

```
2 + √16 × 3          → 14
π × 5²               → 78.539... (圆的面积)
√(3² + 4²)           → 5 (勾股定理)
(4 ÷ 3) × π × 3³     → 113.097... (球体体积)
```

## Unicode 符号输入 ⌨️

### macOS
- `π`: Option + P
- `×`: 输入法搜索 "乘"
- `÷`: 输入法搜索 "除"
- `√`: Option + V 或搜索 "根号"
- `²`: 搜索 "平方"
- `³`: 搜索 "立方"

### 通用方法
所有 Unicode 符号都有 ASCII 等价形式：
- `√16` = `sqrt(16)`
- `5²` = `5 ^ 2`
- `π` = `3.141592653589793`
- `×` = `*`
- `÷` = `/`

## 测试 ✅

```bash
# 运行所有测试
cargo test -p chapter02 --lib

# 运行特定测试
cargo test -p chapter02 --lib evaluate_expression
cargo test -p chapter02 --lib unicode
cargo test -p chapter02 --lib parser
```

当前测试覆盖：**46 个测试，100% 通过率**

## 示例程序 📂

```bash
# 交互式表达式计算器
cargo run -p chapter02 --example expression_calculator

# 运行 Unicode 符号演示
./chapter02/demo_unicode.sh

# 其他示例
cargo run -p chapter02 --example float_types
cargo run -p chapter02 --example integer_types
cargo run -p chapter02 --example boolean_types
```

## 文档 📚

- [完整使用指南](EXPRESSION_EVALUATOR.md) - 详细的功能说明和 API 文档
- [Unicode 符号参考](UNICODE_MATH_SYMBOLS.md) - 所有支持的 Unicode 数学符号
- [更新日志](CHANGELOG.md) - 版本历史和功能变更

## 项目结构 📁

```
chapter02/
├── src/
│   ├── lib.rs              # 库入口
│   ├── main.rs             # 主程序
│   ├── calculator/
│   │   ├── mod.rs          # Calculator 核心
│   │   ├── parser.rs       # 表达式解析器
│   │   ├── evaluator.rs    # 表达式求值器
│   │   └── operations.rs   # 基础运算
│   ├── data/               # 数据类型
│   ├── history/            # 历史记录
│   └── utils/              # 工具函数
├── examples/               # 示例程序
│   └── expression_calculator.rs
├── EXPRESSION_EVALUATOR.md
├── UNICODE_MATH_SYMBOLS.md
└── CHANGELOG.md
```

## 架构设计 🏗️

表达式求值采用经典的三阶段编译器架构：

```
"2 + 3 * 4"
    ↓
[词法分析] Tokenizer
    ↓
[Number(2), Op('+'), Number(3), Op('*'), Number(4)]
    ↓
[语法分析] Parser (递归下降)
    ↓
BinaryOp {
    op: '+',
    left: Number(2),
    right: BinaryOp {
        op: '*',
        left: Number(3),
        right: Number(4)
    }
}
    ↓
[求值] Evaluator
    ↓
14.0
```

## 性能 ⚡

- **时间复杂度**: O(n)，n 为表达式长度
- **空间复杂度**: O(n)，用于存储 AST
- 适用于绝大多数实际应用场景

## 错误处理 ⚠️

完善的错误类型和友好的错误信息：

```rust
match calc.evaluate_expression("1 / 0") {
    Ok(result) => println!("Result: {}", result),
    Err(e) => println!("Error: {}", e), // "除数不能为零"
}
```

支持的错误类型：
- `DivisionByZero` - 除以零
- `NegativeSquareRoot` - 负数开方
- `InvalidLogarithm` - 无效对数
- `ParseError` - 解析错误
- `InvalidCharacter` - 无效字符
- `UndefinedFunction` - 未定义函数
- 更多...

## 未来计划 🎯

- [ ] 变量支持：`x = 5; x + 3`
- [ ] 更多常量：`e`, `φ` (黄金比例)
- [ ] 更多函数：反三角函数、双曲函数
- [ ] 多参数函数：支持逗号分隔
- [ ] 自定义函数定义
- [ ] 更多 Unicode 符号：`∑`, `∏`, `∫`
- [ ] 复数运算
- [ ] 单位转换

## 贡献 🤝

欢迎提交 Issue 和 Pull Request！

添加新功能的步骤：
1. 在 `parser.rs` 中添加词法/语法支持
2. 在 `evaluator.rs` 中添加求值逻辑
3. 添加测试用例
4. 更新文档

## 许可证 📄

MIT License

---

**提示**: 如果你遇到了 `Error: ParseError("表达式求值功能暂未实现")`，说明你使用的是旧版本。请更新到最新版本！✨
