# Unicode 数学符号快速参考

本文档列出了表达式计算器支持的所有 Unicode 数学符号。

## 支持的符号

### 运算符

| 符号 | ASCII 等价 | 名称 | 示例 | 结果 |
|------|-----------|------|------|------|
| `×` | `*` | 乘法 | `5 × 3` | 15 |
| `÷` | `/` | 除法 | `10 ÷ 2` | 5 |
| `√` | `sqrt()` | 平方根 | `√16` | 4 |
| `²` | `^2` | 平方 | `5²` | 25 |
| `³` | `^3` | 立方 | `3³` | 27 |

### 常量

| 符号 | 名称 | 值 | 示例 | 结果 |
|------|------|-----|------|------|
| `π` | 圆周率 | 3.141592653589793 | `2 × π` | 6.283... |

## 使用示例

### 基本运算

```rust
// 使用 Unicode 乘法符号
let calc = Calculator::new();
calc.evaluate_expression("5 × 3")?;  // 15.0

// 使用 Unicode 除法符号
calc.evaluate_expression("20 ÷ 4")?;  // 5.0
```

### 平方根

```rust
// 传统方式
calc.evaluate_expression("sqrt(16)")?;  // 4.0

// Unicode 方式（更简洁）
calc.evaluate_expression("√16")?;  // 4.0

// 可以嵌套使用
calc.evaluate_expression("√(16 + 9)")?;  // 5.0

// 与其他运算组合
calc.evaluate_expression("2 + √16 × 3")?;  // 14.0
```

### 幂运算

```rust
// 传统平方
calc.evaluate_expression("5 ^ 2")?;  // 25.0

// Unicode 平方（更简洁）
calc.evaluate_expression("5²")?;  // 25.0

// 传统立方
calc.evaluate_expression("3 ^ 3")?;  // 27.0

// Unicode 立方（更简洁）
calc.evaluate_expression("3³")?;  // 27.0
```

### 圆周率

```rust
// 使用 π 常量
calc.evaluate_expression("π")?;  // 3.141592653589793

// 计算圆的周长：C = 2πr (r=5)
calc.evaluate_expression("2 × π × 5")?;  // 31.415...

// 计算圆的面积：A = πr² (r=5)
calc.evaluate_expression("π × 5²")?;  // 78.539...
```

### 复杂表达式

```rust
// 混合使用多种 Unicode 符号
calc.evaluate_expression("√(3² + 4²)")?;  // 5.0 (勾股定理)

// 球体体积：V = (4/3)πr³ (r=3)
calc.evaluate_expression("(4 ÷ 3) × π × 3³")?;  // 113.097...

// 二次方程判别式：Δ = b² - 4ac (a=1, b=5, c=6)
calc.evaluate_expression("5² - 4 × 1 × 6")?;  // 1.0
```

## 输入方法

### macOS

- `π`: Option + P
- `×`: Option + Shift + 9 (或输入法中搜索"乘")
- `÷`: Option + / (或输入法中搜索"除")
- `√`: Option + V (或输入法中搜索"根号")
- `²`: Option + 2 (在某些键盘布局中，或输入法中搜索"平方")
- `³`: Option + 3 (在某些键盘布局中，或输入法中搜索"立方")

### Windows

- 使用字符映射表 (charmap.exe)
- 或使用 Alt 代码：
  - `π`: Alt + 227
  - `×`: Alt + 0215
  - `÷`: Alt + 0247
  - `√`: Alt + 251
  - `²`: Alt + 0178
  - `³`: Alt + 0179

### Linux

- 使用 Compose 键或 Unicode 输入
- 或从字符选择器复制

### 通用方法

如果不方便输入 Unicode 符号，可以直接使用 ASCII 等价形式：

```rust
// 这两种写法完全等价
calc.evaluate_expression("√16")?;      // Unicode
calc.evaluate_expression("sqrt(16)")?; // ASCII

calc.evaluate_expression("5²")?;       // Unicode
calc.evaluate_expression("5 ^ 2")?;    // ASCII

calc.evaluate_expression("π")?;        // Unicode
// π 没有简单的 ASCII 等价形式，但可以直接用数值 3.14159...
```

## 实现细节

### 词法分析

Unicode 符号在词法分析阶段被转换为标准 token：

```rust
"5 × 3"  → [Number(5.0), Operator('*'), Number(3.0)]
"√16"    → [PrefixOp("sqrt"), Number(16.0)]
"5²"     → [Number(5.0), Operator('^'), Number(2.0)]
"π"      → [Constant("pi")]
```

### 优先级

Unicode 符号遵循与其 ASCII 等价形式相同的优先级规则：

1. 括号 `()`
2. 前缀运算符 `√`
3. 一元运算符 `+`, `-`
4. 幂运算 `^`, `²`, `³` (右结合)
5. 乘除 `*`, `/`, `×`, `÷`
6. 加减 `+`, `-`

## 常见问题

### Q: 为什么不支持更多的 Unicode 符号？

A: 当前版本专注于最常用的数学符号。未来版本可能会添加更多符号，如：
- `∑` (求和)
- `∏` (乘积)
- `∫` (积分)
- `∂` (偏微分)
- `∞` (无穷大)

### Q: 能否混用 Unicode 和 ASCII 符号？

A: 当然可以！例如：

```rust
calc.evaluate_expression("√16 * 3")?;   // 混用 √ 和 *
calc.evaluate_expression("π × 2")?;     // 混用 π 和 ×
calc.evaluate_expression("5² + sqrt(9)")?; // 混用 ² 和 sqrt()
```

### Q: Unicode 符号是否影响性能？

A: 几乎没有影响。Unicode 符号在词法分析的第一阶段就被转换为标准形式，后续处理完全相同。

### Q: 如何在代码中使用 Unicode 表达式？

A: 在 Rust 源代码中直接使用：

```rust
use chapter02::Calculator;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let calc = Calculator::new();

    // 直接在字符串字面量中使用 Unicode
    let result = calc.evaluate_expression("π × 5²")?;
    println!("圆的面积 (r=5): {}", result);

    Ok(())
}
```

## 测试

运行 Unicode 符号相关的测试：

```bash
# 测试 Unicode tokenization
cargo test -p chapter02 --lib tokenize_unicode

# 测试 Unicode 解析
cargo test -p chapter02 --lib parse_unicode

# 测试所有与 Unicode 相关的功能
cargo test -p chapter02 --lib unicode
```

## 贡献

如果你希望添加更多 Unicode 符号支持，请：

1. 在 `calculator/parser.rs` 的 `tokenize` 方法中添加新符号的识别逻辑
2. 添加相应的测试用例
3. 更新本文档
4. 提交 Pull Request

## 参考资源

- [Unicode 数学符号列表](https://en.wikipedia.org/wiki/Mathematical_operators_and_symbols_in_Unicode)
- [Unicode 字符表](https://unicode-table.com/en/blocks/mathematical-operators/)
- [Rust String 和 char 文档](https://doc.rust-lang.org/std/primitive.char.html)
