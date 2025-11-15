fn pattern_matching() {
    let x = 42;

    match x {
        0 => println!("零"),
        1..=10 => println!("一到十之间"),
        20 | 30 | 40 => println!("20, 30 或 40"),
        n if n % 2 == 0 => println!("偶数: {}", n),
        _ => println!("其他值: {}", x), // 通配符
    }

    // 绑定值的模式
    match x {
        0 => println!("零"),
        1 => println!("一"),
        n => println!("其他: {}", n), // 绑定值
    }

    // 复合模式
    let point = (0, 7);
    match point {
        (0, 0) => println!("原点"),
        (0, y) => println!("在 Y 轴上: y = {}", y),
        (x, 0) => println!("在 X 轴上: x = {}", x),
        (x, y) => println!("点 ({}, {})", x, y),
    }
}
fn main() {
    println!("=== pattern matching example ===");
    pattern_matching();
    println!("=== pattern matching example end! ===");
}
