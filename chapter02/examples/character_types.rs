fn character_types() {
    let c1 = 'z'; // 单个字符
    let c2 = 'ℤ'; // Unicode 字符
    let c3 = '😊'; // 表情符号

    println!("字符: {}, {}, {}", c1, c2, c3);

    // 转义字符
    let newline = '\n';
    let tab = '\t';
    let quote = '\'';
    let backslash = '\\';

    // 字符串中的字符
    let string = "Hello, 世界! 🌍";
    for (index, char) in string.chars().enumerate() {
        println!("字符 {}: {}", index, char);
    }

    // 获取字节
    let bytes = string.as_bytes();
    println!("字符串长度（字节）: {}", bytes.len());
}

fn main() {
    println!("=== character types example ===");
    character_types();
    println!("=== character types example end! ===");
}
