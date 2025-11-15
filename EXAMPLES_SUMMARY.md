# Examples 快速总结

## 如何创建和运行示例

### 方式一：单文件示例（推荐用于简单示例）

**创建：**
```bash
# 直接在 examples/ 目录下创建 .rs 文件
touch chapter02/examples/my_example.rs
```

**运行：**
```bash
cargo run -p chapter02 --example my_example
```

**结构：**
```
chapter02/examples/my_example.rs  ← 单个文件
```

---

### 方式二：多文件示例（用于复杂项目）

**创建：**
```bash
# 创建目录和 main.rs
mkdir -p chapter02/examples/my_project
touch chapter02/examples/my_project/main.rs
touch chapter02/examples/my_project/utils.rs
```

**运行：**
```bash
cargo run -p chapter02 --example my_project
```

**结构：**
```
chapter02/examples/my_project/
├── main.rs      ← 必须命名为 main.rs
├── utils.rs
└── config.rs
```

---

## 现有示例

### Chapter 01
```bash
cargo run -p chapter01 --example variables
cargo run -p chapter01 --example functions
cargo run -p chapter01 --example control_flow
```

### Chapter 02
```bash
# 单文件示例（部分）
cargo run -p chapter02 --example variable_basics
cargo run -p chapter02 --example boolean_types
cargo run -p chapter02 --example integer_types
cargo run -p chapter02 --example float_types
cargo run -p chapter02 --example array_basics
# ... 等 27 个单文件示例

# 多文件示例
cargo run -p chapter02 --example complex_example
```

---

## 关键点

1. **单文件示例**：文件名 = 示例名（不含 .rs）
   - `my_example.rs` → `--example my_example`

2. **多文件示例**：目录名 = 示例名，必须有 main.rs
   - `my_project/main.rs` → `--example my_project`

3. **模块声明**：在 main.rs 中使用 `mod` 声明其他文件
   ```rust
   mod utils;
   mod config;
   ```

4. **查看所有示例**：
   ```bash
   ls chapter02/examples/
   ```

---

详细指南请参考：[EXAMPLES_GUIDE.md](EXAMPLES_GUIDE.md)
