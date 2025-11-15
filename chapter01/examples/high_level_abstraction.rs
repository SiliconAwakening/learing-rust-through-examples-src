// Chapter 01 示例2: 函数的使用
// 演示 Rust 中函数的定义、参数、返回值等特性

fn main() {
    println!("=== 示例2: 函数的使用 ===\n");

    // 1. 基本函数调用
    println!("1. 基本函数:");
    greet("Rust");

    // 2. 带返回值的函数
    println!("\n2. 带返回值的函数:");
    let sum = add(15, 27);
    println!("   15 + 27 = {}", sum);

    let product = multiply(6, 7);
    println!("   6 × 7 = {}", product);

    // 3. 多个返回值（使用元组）
    println!("\n3. 返回多个值:");
    let (quotient, remainder) = divide(17, 5);
    println!("   17 ÷ 5 = {} 余 {}", quotient, remainder);

    // 4. 表达式作为返回值
    println!("\n4. 表达式返回值:");
    let max = max_value(42, 28);
    println!("   max(42, 28) = {}", max);

    // 5. 没有返回值的函数（返回单元类型）
    println!("\n5. 无返回值函数:");
    print_separator();

    // 6. 函数作为参数
    println!("\n6. 高阶函数:");
    let result = apply_operation(10, 5, add);
    println!("   apply(10, 5, add) = {}", result);

    let result = apply_operation(10, 5, multiply);
    println!("   apply(10, 5, multiply) = {}", result);

    // 7. 递归函数
    println!("\n7. 递归函数:");
    let n = 5;
    println!("   {}! = {}", n, factorial(n));

    println!("\n=== 示例运行完成 ===");
}

/// 简单的问候函数
fn greet(name: &str) {
    println!("   Hello, {}!", name);
}

/// 加法函数
fn add(a: i32, b: i32) -> i32 {
    a + b // 最后一个表达式作为返回值，不需要 return 和分号
}

/// 乘法函数
fn multiply(a: i32, b: i32) -> i32 {
    return a * b; // 也可以显式使用 return
}

/// 除法函数，返回商和余数
fn divide(dividend: i32, divisor: i32) -> (i32, i32) {
    let quotient = dividend / divisor;
    let remainder = dividend % divisor;
    (quotient, remainder)
}

/// 返回两个数中的较大值
fn max_value(a: i32, b: i32) -> i32 {
    if a > b {
        a
    } else {
        b
    }
}

/// 打印分割线
fn print_separator() {
    println!("   {}", "-".repeat(40));
}

/// 高阶函数：接受函数作为参数
fn apply_operation(a: i32, b: i32, operation: fn(i32, i32) -> i32) -> i32 {
    operation(a, b)
}

/// 递归函数：计算阶乘
fn factorial(n: u32) -> u32 {
    if n == 0 {
        1
    } else {
        n * factorial(n - 1)
    }
}
