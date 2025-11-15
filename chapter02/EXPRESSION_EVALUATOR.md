# 表达式求值器 - 使用说明

## 概述

Chapter 02 现在包含了一个完整的表达式求值器，可以解析和计算数学表达式。该求值器支持基本运算符、函数调用、括号以及正确的运算符优先级。

## 功能特性

### 支持的运算符

- **加法**: `+`
- **减法**: `-`
- **乘法**: `*` 或 `×` (Unicode)
- **除法**: `/` 或 `÷` (Unicode)
- **幂运算**: `^` (右结合，例如 `2^3^2` = 512)
- **平方**: `²` (上标，例如 `5²` = 25)
- **立方**: `³` (上标，例如 `3³` = 27)

### 支持的函数

- **三角函数**: `sin(x)`, `cos(x)`, `tan(x)`
- **平方根**: `sqrt(x)` 或 `√x` (Unicode，例如 `√16` = 4)
- **对数**: `ln(x)` (自然对数), `log(value, base)` (指定底数)
- **绝对值**: `abs(x)`
- **取整**: `ceil(x)`, `floor(x)`, `round(x)`

### 支持的常量

- **圆周率**: `π` (pi ≈ 3.14159...)
- **自然常数**: 可通过函数访问 (例如: `ln(e)` 或使用值 2.71828...)

### 其他特性

- **括号**: 使用 `()` 改变运算优先级
- **一元运算符**: 负号 `-` 和正号 `+`
- **Unicode 数学符号**: 支持 `√`, `π`, `×`, `÷`, `²`, `³` 等
- **运算符优先级**:
  1. 括号 `()`
  2. 函数调用 `func()` 和前缀运算符 `√`
  3. 一元运算符 `+`, `-`
  4. 幂运算 `^`, `²`, `³` (右结合)
  5. 乘除 `*`, `/`, `×`, `÷`
  6. 加减 `+`, `-`

## 使用方法

### 作为库使用

```rust
use chapter02::Calculator;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let calc = Calculator::new();

    // 简单表达式
    let result = calc.evaluate_expression("2 + 3")?;
    println!("2 + 3 = {}", result); // 输出: 5

    // 运算符优先级
    let result = calc.evaluate_expression("2 + 3 * 4")?;
    println!("2 + 3 * 4 = {}", result); // 输出: 14

    // 括号
    let result = calc.evaluate_expression("(2 + 3) * 4")?;
    println!("(2 + 3) * 4 = {}", result); // 输出: 20

    // 函数调用
    let result = calc.evaluate_expression("sqrt(16)")?;
    println!("sqrt(16) = {}", result); // 输出: 4

    // 复杂表达式
    let result = calc.evaluate_expression("2 + sqrt(16) * 3")?;
    println!("2 + sqrt(16) * 3 = {}", result); // 输出: 14

    Ok(())
}
```

### 运行交互式计算器

```bash
# 运行表达式计算器示例
cargo run -p chapter02 --example expression_calculator

# 或者使用管道输入表达式
echo "2 + 3 * 4" | cargo run -p chapter02 --example expression_calculator
```

## 表达式示例

| 表达式              | 结果       | 说明                   |
| ------------------- | ---------- | ---------------------- |
| `2 + 3`             | 5          | 简单加法               |
| `10 - 4 * 2`        | 2          | 乘法优先级高于减法     |
| `(10 - 4) * 2`      | 12         | 括号改变优先级         |
| `2 ^ 3`             | 8          | 幂运算                 |
| `2 ^ 3 ^ 2`         | 512        | 幂运算右结合 (2^(3^2)) |
| `sqrt(16)`          | 4          | 平方根函数             |
| `sin(0)`            | 0          | 正弦函数               |
| `2 + sqrt(16) * 3`  | 14         | 混合运算               |
| `(2 + 3) * (4 + 5)` | 45         | 多组括号               |
| `-5 + 10`           | 5          | 一元负号               |
| `abs(-42)`          | 42         | 绝对值                 |
| `ceil(3.14)`        | 4          | 向上取整               |
| `floor(3.99)`       | 3          | 向下取整               |
| `√16`               | 4          | Unicode 平方根         |
| `5²`                | 25         | Unicode 平方           |
| `3³`                | 27         | Unicode 立方           |
| `π`                 | 3.14159... | Unicode 圆周率         |
| `2 × π`             | 6.28318... | Unicode 乘法           |
| `10 ÷ 5`            | 2          | Unicode 除法           |
| `√(16 + 9)`         | 5          | 组合使用               |
| `π × 5²`            | 78.539...  | 复杂 Unicode 表达式    |

## 架构设计

表达式求值器采用经典的三阶段架构：

### 1. 词法分析 (Tokenization)

将输入字符串转换为 Token 序列：

```rust
"2 + 3" → [Number(2.0), Operator('+'), Number(3.0)]
```

### 2. 语法分析 (Parsing)

使用递归下降解析器将 Token 序列转换为抽象语法树 (AST)：

```rust
[Number(2.0), Operator('+'), Number(3.0)]
  ↓
BinaryOp {
    op: '+',
    left: Number(2.0),
    right: Number(3.0)
}
```

### 3. 求值 (Evaluation)

递归遍历 AST 并计算结果：

```rust
BinaryOp { op: '+', left: Number(2.0), right: Number(3.0) }
  → evaluate(left) + evaluate(right)
  → 2.0 + 3.0
  → 5.0
```

## 错误处理

求值器会返回详细的错误信息：

```rust
match calc.evaluate_expression("1 / 0") {
    Ok(result) => println!("Result: {}", result),
    Err(e) => println!("Error: {}", e), // 输出: "Error: 除数不能为零"
}
```

常见错误类型：

- `DivisionByZero`: 除以零
- `NegativeSquareRoot`: 对负数开平方根
- `InvalidLogarithm`: 无效的对数参数
- `ParseError`: 解析错误（如括号不匹配）
- `InvalidCharacter`: 无效字符
- `InvalidOperator`: 无效运算符
- `UndefinedFunction`: 未定义的函数
- `InvalidArgumentCount`: 函数参数数量错误

## 测试

运行所有测试：

```bash
# 运行所有库测试
cargo test -p chapter02 --lib

# 运行特定测试
cargo test -p chapter02 --lib evaluate_expression
cargo test -p chapter02 --lib parser::tests
cargo test -p chapter02 --lib evaluator::tests
```

当前测试覆盖：

- ✅ 基本运算符测试
- ✅ 运算符优先级测试
- ✅ 括号测试
- ✅ 函数调用测试
- ✅ 一元运算符测试
- ✅ 复杂表达式测试
- ✅ 错误处理测试

## 未来扩展

可能的功能扩展方向：

1. **变量支持**: 允许定义和使用变量 (`x = 5; x + 3`)
2. **更多常量**: 添加更多内置常量如 `e`, `φ` (黄金比例) 等
3. **更多函数**: 三角函数反函数、双曲函数等
4. **多参数函数**: 支持逗号分隔的多个参数
5. **自定义函数**: 允许用户定义函数
6. **单位转换**: 支持物理单位 (`5m + 3cm`)
7. **复数**: 支持复数运算
8. **更多 Unicode 符号**: `∑` (求和), `∏` (乘积), `∫` (积分) 等

## 性能考虑

- 词法分析时间复杂度: O(n)，n 为输入字符串长度
- 语法分析时间复杂度: O(n)，n 为 token 数量
- 求值时间复杂度: O(n)，n 为 AST 节点数量
- 总体时间复杂度: O(n)

对于大多数实际应用场景，性能足够优秀。

## 贡献指南

如需添加新功能：

1. 在 `calculator/parser.rs` 中添加新的 Token 类型或 AstNode 变体
2. 在解析器中添加相应的解析逻辑
3. 在 `calculator/evaluator.rs` 中添加求值逻辑
4. 添加测试用例
5. 更新本文档

## 许可证

本项目遵循 MIT 许可证。
