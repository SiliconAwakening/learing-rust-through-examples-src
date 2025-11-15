fn while_examples() {
    let mut number = 3;

    while number != 0 {
        println!("倒计时: {}", number);
        number -= 1;
    }

    println!("发射！");

    // 数组遍历
    let array = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < array.len() {
        println!("索引 {}: 值 {}", index, array[index]);
        index += 1;
    }
}
fn main() {
    println!("=== while examples ===");
    while_examples();
    println!("=== while examples end! ===");
}
