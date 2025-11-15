// 基础函数
fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn multiply(a: i32, b: i32) -> i32 {
    a * b
}

// 函数作为参数
fn apply_function<F>(value: i32, f: F) -> i32
where
    F: Fn(i32) -> i32,
{
    f(value)
}

// 函数作为返回值
fn get_operation(operation: &str) -> fn(i32, i32) -> i32 {
    match operation {
        "add" => add,
        "multiply" => multiply,
        _ => add, // 默认操作
    }
}

fn higher_order_functions() {
    let result1 = apply_function(5, |x| x * x); // 闭包
    let result2 = apply_function(10, |x| x + 100); // 另一个闭包

    println!("平方: {}", result1);
    println!("加100: {}", result2);

    let operation = get_operation("add");
    let result3 = operation(15, 25);
    println!("函数指针结果: {}", result3);
}
fn main() {
    println!("=== higher order functions example ===");
    higher_order_functions();
    println!("=== higher order functions example end! ===");
}
