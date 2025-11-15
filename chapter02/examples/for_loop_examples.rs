fn for_loop_examples() {
    // 基本 for 循环
    for i in 0..5 {
        // 0 到 4
        println!("for 循环: {}", i);
    }

    // 包含结束值的 range
    for i in 0..=5 {
        // 0 到 5
        println!("包含结束值: {}", i);
    }

    // 数组迭代
    let array = [1, 2, 3, 4, 5];
    for item in array {
        println!("数组项: {}", item);
    }

    // 索引和值的迭代
    let names = vec!["Alice", "Bob", "Charlie"];
    for (index, name) in names.iter().enumerate() {
        println!("{}: {}", index, name);
    }

    // 字符串字符迭代
    let text = "Rust";
    for char in text.chars() {
        println!("字符: {}", char);
    }

    // 字节迭代
    for byte in text.bytes() {
        println!("字节: {}", byte);
    }
}
fn main() {
    println!("=== for loop examples ===");
    for_loop_examples();
    println!("=== for loop examples end! ===");
}
