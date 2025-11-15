fn high_level_abstraction() {
    let numbers: Vec<i32> = (0..1000).collect();

    // 高级的函数式编程风格
    let sum: i32 = numbers
        .iter()
        .filter(|&&x| x % 2 == 0) // 只保留偶数
        .map(|&x| x * x) // 平方
        .sum(); // 求和

    // 编译器会优化为类似 C 代码的性能
    println!("平方偶数和: {}", sum);
}

fn main() {
    println!("=== high level abstraction example ===");
    high_level_abstraction();
    println!("=== high level abstraction example end! ===");
}
