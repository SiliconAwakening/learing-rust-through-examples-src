fn string_and_char_arrays() {
    // 字符串数组
    let fruits = ["苹果", "香蕉", "橙子", "葡萄"];

    for (i, fruit) in fruits.iter().enumerate() {
        println!("fruits[{}] = {}", i, fruit);
    }

    // 字符数组
    let word = ['R', 'u', 's', 't'];
    let word_str: String = word.iter().collect();
    println!("字符数组转换为字符串: {}", word_str);

    // 字符数组的遍历
    for char in &word {
        println!("字符: {}", char);
        // 转换为ASCII码
        println!("ASCII码: {}", *char as u8);
    }

    // 计算字符串的长度（以字符计）
    let multi_char_str = "你好，世界！ 🌍";
    let chars: Vec<char> = multi_char_str.chars().collect();
    println!("字符串: {}", multi_char_str);
    println!("字符数量: {}", chars.len());
    println!("字节长度: {}", multi_char_str.len());
}

fn main() {
    println!("=== string and char arrays example ===");
    string_and_char_arrays();
    println!("=== string and char arrays example end! ===");
}
