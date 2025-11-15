// Chapter 01: Rust 基础入门
// 本章节演示 Rust 的基本概念

fn main() {
    println!("=== Chapter 01: Rust 基础入门 ===\n");

    // 1. 变量和可变性
    println!("1. 变量和可变性:");
    let x = 5;
    println!("   不可变变量 x = {}", x);

    let mut y = 10;
    println!("   可变变量 y = {}", y);
    y = 15;
    println!("   修改后 y = {}", y);

    // 2. 数据类型
    println!("\n2. 基本数据类型:");
    let integer: i32 = 42;
    let float: f64 = 3.14;
    let boolean: bool = true;
    let character: char = '中';
    println!(
        "   整数: {}, 浮点数: {}, 布尔值: {}, 字符: {}",
        integer, float, boolean, character
    );

    // 3. 函数调用
    println!("\n3. 函数调用:");
    let result = add(10, 20);
    println!("   10 + 20 = {}", result);

    // 4. 控制流
    println!("\n4. 控制流:");
    let number = 7;
    if number < 10 {
        println!("   {} 小于 10", number);
    } else {
        println!("   {} 大于等于 10", number);
    }

    // 5. 循环
    println!("\n5. 循环示例:");
    print!("   斐波那契数列前10项: ");
    for i in 0..10 {
        print!("{} ", fibonacci(i));
    }
    println!();

    println!("\n=== Chapter 01 主程序运行完成 ===");
    println!("\n提示: 运行 examples 中的示例:");
    println!("  cargo run -p chapter01 --example demonstrate_memory_safety");
    println!("  cargo run -p chapter01 --example high_level_abstraction");
    println!("  cargo run -p chapter01 --example ownership_demo");
}

/// 加法函数
fn add(a: i32, b: i32) -> i32 {
    a + b
}

/// 计算斐波那契数列
fn fibonacci(n: u32) -> u32 {
    match n {
        0 => 0,
        1 => 1,
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    }
}
