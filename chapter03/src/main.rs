// Chapter 03: 所有权系统
// 本章节演示 Rust 的核心特性：所有权、借用和生命周期

fn main() {
    println!("=== Chapter 03: 所有权系统 ===\n");

    // 1. 所有权基础
    println!("1. 所有权转移:");
    let s1 = String::from("Hello");
    println!("   创建字符串 s1: {}", s1);

    let s2 = s1; // 所有权转移给 s2
    println!("   所有权转移到 s2: {}", s2);
    // println!("   {}", s1); // 错误！s1 已失效

    // 2. 克隆（深拷贝）
    println!("\n2. 克隆数据:");
    let s3 = String::from("World");
    let s4 = s3.clone();
    println!("   s3: {}, s4: {}", s3, s4);

    // 3. 引用和借用
    println!("\n3. 不可变引用:");
    let s5 = String::from("Rust");
    let len = calculate_length(&s5);
    println!("   字符串 '{}' 的长度是 {}", s5, len);

    // 4. 可变引用
    println!("\n4. 可变引用:");
    let mut s6 = String::from("Hello");
    println!("   修改前: {}", s6);
    change(&mut s6);
    println!("   修改后: {}", s6);

    // 5. 切片
    println!("\n5. 字符串切片:");
    let text = String::from("Hello World");
    let hello = &text[0..5];
    let world = &text[6..11];
    println!("   原字符串: {}", text);
    println!("   切片1: {}, 切片2: {}", hello, world);

    // 6. 所有权与函数
    println!("\n6. 所有权与函数:");
    let s7 = String::from("Function Test");
    println!("   传递前: {}", s7);
    takes_ownership(s7);
    // println!("   {}", s7); // 错误！所有权已转移

    let x = 42;
    println!("   传递数字: {}", x);
    makes_copy(x);
    println!("   数字仍可用: {}", x); // i32 实现了 Copy trait

    println!("\n=== Chapter 03 主程序运行完成 ===");
    println!("\n提示: 运行 examples 中的示例:");
    println!("  cargo run -p chapter03 --example ownership");
    println!("  cargo run -p chapter03 --example borrowing");
    println!("  cargo run -p chapter03 --example slices");
}

/// 计算字符串长度（借用，不获取所有权）
fn calculate_length(s: &String) -> usize {
    s.len()
}

/// 修改字符串（可变借用）
fn change(s: &mut String) {
    s.push_str(", World!");
}

/// 获取所有权的函数
fn takes_ownership(some_string: String) {
    println!("   函数内部: {}", some_string);
} // some_string 在这里被释放

/// 复制值的函数
fn makes_copy(some_integer: i32) {
    println!("   函数内部: {}", some_integer);
}
