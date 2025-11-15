// 常量声明（always immutable, must have type annotation）
fn constants_example() {
    const PI: f64 = 3.14159265359; // 浮点数常量
    const MAX_SIZE: usize = 1000; // 无符号整数常量
    const GREETING: &str = "Hello, World!"; // 字符串常量

    println!("PI = {}", PI);
    println!("最大尺寸: {}", MAX_SIZE);
    println!("问候语: {}", GREETING);

    // 常量表达式
    const AREA: f64 = PI * 10.0 * 10.0; // 圆面积公式
    println!("圆面积: {}", AREA);
}

fn main() {
    println!("=== constants example ===");
    constants_example();
    println!("=== constants example end! ===");
}
