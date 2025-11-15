fn tuple_basics() {
    // 创建元组
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let tup2 = (42, "Hello", true);

    // 访问元组元素（使用索引）
    let x = tup.0; // 500
    let y = tup.1; // 6.4
    let z = tup.2; // 1

    println!("元组值: ({}, {}, {})", x, y, z);

    // 解构赋值（模式匹配）
    let (a, b, c) = tup;
    println!("解构后的值: a={}, b={}, c={}", a, b, c);

    // 单个元素的元组（注意逗号）
    let single_tuple: (i32,) = (5,);
    let single_value = single_tuple.0;
    println!("单个元素元组: ({}, {})", single_value, single_tuple.0);
}

fn main() {
    println!("=== tuple basics example ===");
    tuple_basics();
    println!("=== tuple basics example end! ===");
}
