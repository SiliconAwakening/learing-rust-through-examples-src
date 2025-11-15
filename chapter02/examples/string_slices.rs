fn string_slices() {
    // 字符串字面量（&str）- 编译时常量
    let greeting = "Hello, Rust!";
    let name = "World";

    // 切片（不拥有所有权）
    let slice = &greeting[0..5]; // "Hello"
    let slice_from_middle = &greeting[7..11]; // "Rust"

    println!("完整问候: {}", greeting);
    println!("切片: {}", slice);

    // 字符串方法
    let trimmed = "  hello  ".trim(); // "hello"
    let uppercase = "rust".to_uppercase(); // "RUST"
    let lowercase = "RUST".to_lowercase(); // "rust"

    // 查找和分割
    let text = "one,two,three,four";
    let parts: Vec<&str> = text.split(',').collect();
    println!("分割结果: {:?}", parts);

    // 替换
    let replaced = "hello world".replace("world", "Rust");
    println!("替换后: {}", replaced);
}

fn main() {
    println!("=== string slices example ===");
    string_slices();
    println!("=== string slices example end! ===");
}
