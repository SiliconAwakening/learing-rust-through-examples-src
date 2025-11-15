fn boolean_types() {
    let is_learning_rust = true;
    let is_difficult = false;

    // 条件表达式
    let message = if is_learning_rust {
        "Keep going!"
    } else {
        "Try harder!"
    };

    // 布尔逻辑
    let both_true = is_learning_rust && !is_difficult;
    let either_or = is_learning_rust || is_difficult;

    println!("{} {}", message, both_true);
    println!("Either learning or difficult: {}", either_or);

    // 模式匹配中的布尔
    match (is_learning_rust, is_difficult) {
        (true, false) => println!("Perfect learning situation!"),
        (true, true) => println!("Challenging but rewarding!"),
        (false, _) => println!("Maybe try something else?"),
    }
}

fn main() {
    println!("=== boolean types example ===");
    boolean_types();
    println!("=== boolean types example end! ===");
}
