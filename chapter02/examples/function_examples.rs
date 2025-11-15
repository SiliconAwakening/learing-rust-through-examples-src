// 函数定义
fn greet(name: &str) {
    println!("你好, {}!", name);
}

// 有返回值的函数
fn add(a: i32, b: i32) -> i32 {
    a + b // 没有分号表示返回值
}

// 显式返回语句
fn multiply(x: i32, y: i32) -> i32 {
    return x * y; // 显式返回
}

// 函数调用
fn function_examples() {
    greet("Rust");
    let sum = add(5, 3);
    let product = multiply(4, 7);

    println!("5 + 3 = {}", sum);
    println!("4 × 7 = {}", product);

    // 函数作为表达式
    let result = {
        let a = 10;
        let b = 20;
        a + b // 块表达式返回值
    };
    println!("块表达式结果: {}", result);
}
fn main() {
    println!("=== function examples ===");
    function_examples();
    println!("=== function examples end! ===");
}
