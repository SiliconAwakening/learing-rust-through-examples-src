fn string_type() {
    // String 类型（拥有所有权）
    let mut s = String::new(); // 空字符串
    let s1 = String::from("hello"); // 从字符串字面量创建
    let s2 = "world".to_string(); // 转换为 String

    // 追加内容
    s.push('A'); // 追加字符
    s.push_str("pple"); // 追加字符串
    s += " Banana"; // 连接操作符

    println!("字符串 s: {}", s);

    // 格式化
    let name = "Alice";
    let age = 30;
    let formatted = format!("{} is {} years old", name, age);
    println!("格式化: {}", formatted);

    // 使用宏
    println!("测试值: {}, 另一个值: {}", 42, "text");

    // 所有权示例
    let original = String::from("original");
    let moved = original; // 所有权转移
                          // println!("{}", original);           // 编译错误！original 已移动
    println!("移动后的字符串: {}", moved);
}

fn main() {
    println!("=== string type example ===");
    string_type();
    println!("=== string type example end! ===");
}
