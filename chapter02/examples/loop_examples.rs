fn loop_examples() {
    // 基本 loop
    let mut counter = 0;
    loop {
        counter += 1;
        println!("计数器: {}", counter);

        if counter >= 5 {
            break; // 退出循环
        }
    }

    // loop 作为表达式（返回值）
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter; // 退出并返回值
        }
    };

    println!("循环结果: {}", result);
}

fn main() {
    println!("=== loop examples ===");
    loop_examples();
    println!("=== loop examples end! ===");
}
