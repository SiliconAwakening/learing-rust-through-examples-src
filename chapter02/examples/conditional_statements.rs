fn conditional_statements() {
    let number = 7;

    // 基本 if-else
    if number < 5 {
        println!("数字小于 5");
    } else if number == 5 {
        println!("数字等于 5");
    } else {
        println!("数字大于 5");
    }

    // if 作为表达式（返回值）
    let grade = if number >= 90 {
        "A"
    } else if number >= 80 {
        "B"
    } else if number >= 70 {
        "C"
    } else {
        "F"
    };

    println!("成绩: {}", grade);

    // 条件赋值
    let status = if number % 2 == 0 { "偶数" } else { "奇数" };
    println!("{} 是 {}", number, status);
}

fn main() {
    println!("=== conditional statements example ===");
    conditional_statements();
    println!("=== conditional statements example end! ===");
}
