fn practical_tuples() {
    // 函数返回多个值
    let result = divide_and_remainder(17, 5);
    let (quotient, remainder) = result;
    println!("17 除以 5 的商和余数: {}, {}", quotient, remainder);

    // 使用解构直接获取结果
    let (sum, product) = calculate_sum_product(10, 20);
    println!("和: {}, 积: {}", sum, product);

    // 存储混合类型的数据
    let person_info = ("张三", 25, 175.5, true);
    let (name, age, height, is_student) = person_info;
    println!(
        "{}今年{}岁，身高{:.1}cm，状态：{}",
        name,
        age,
        height,
        if is_student { "学生" } else { "非学生" }
    );

    // 嵌套元组
    let nested_tuple = (1, (2, 3), 4);
    let inner_tuple = nested_tuple.1;
    let first_inner = inner_tuple.0; // 2
    println!("嵌套元组中的值: {}", first_inner);
}

// 返回元组的函数示例
fn divide_and_remainder(dividend: i32, divisor: i32) -> (i32, i32) {
    let quotient = dividend / divisor;
    let remainder = dividend % divisor;
    (quotient, remainder)
}

fn calculate_sum_product(a: i32, b: i32) -> (i32, i32) {
    (a + b, a * b)
}

fn main() {
    println!("=== practical tuples example ===");
    practical_tuples();
    println!("=== practical tuples example end! ===");
}
