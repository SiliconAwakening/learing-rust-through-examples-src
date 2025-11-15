fn tuple_pattern_matching() {
    let coordinates = (10, 20);

    match coordinates {
        (0, 0) => println!("原点"),
        (x, 0) => println!("在X轴上，X坐标: {}", x),
        (0, y) => println!("在Y轴上，Y坐标: {}", y),
        (x, y) => println!("坐标点: ({}, {})", x, y),
    }

    // 包含条件守卫的模式
    let point = (15, 30);
    match point {
        (x, y) if x == y => println!("在对角线上: ({}, {})", x, y),
        (x, y) if x + y == 45 => println!("坐标和为45: ({}, {})", x, y),
        (x, y) => println!("一般坐标: ({}, {})", x, y),
    }

    // 解构函数参数
    let (name, age) = get_person_info();
    println!("个人信息: {}，{}岁", name, age);
}

fn get_person_info() -> (&'static str, u32) {
    ("李四", 30)
}

fn main() {
    println!("=== tuple pattern matching example ===");
    tuple_pattern_matching();
    println!("=== tuple pattern matching example end! ===");
}
