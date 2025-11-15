// 基本变量声明（不可变）
fn variable_basics() {
    let x = 42; // 整数
    let y = 3.14; // 浮点数
    let name = "Rust"; // 字符串字面量
    let is_rust_awesome = true; // 布尔值

    println!("整数: {}", x);
    println!("浮点数: {}", y);
    println!("字符串: {}", name);
    println!("布尔值: {}", is_rust_awesome);

    // 变量遮蔽（shadowing）
    let x = x + 10; // 创建新的 x，原 x 被隐藏
    {
        let x = "shadowed"; // 创建新的 x，原 x 被隐藏
        println!("遮蔽中的 x: {}", x);
    }
    println!("遮蔽后的 x: {}", x);
}
fn main() {
    println!("=== variable basics ===");
    variable_basics();
    println!("=== variable basics end! ===");
}
