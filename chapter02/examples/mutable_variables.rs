fn mutable_variables() {
    // 可变变量声明
    let mut counter = 0;
    println!("初始值: {}", counter);

    // 修改可变变量
    counter += 1;
    counter *= 2;
    println!("修改后: {}", counter);

    // 可变变量的典型用途
    let mut sum = 0; // 初始化为 0
    let numbers = vec![1, 2, 3, 4, 5]; // 创建一个向量

    for num in numbers {
        sum += num; // 在循环中累积计算
    }

    println!("数列和: {}", sum);
}

fn main() {
    println!("=== mutable variables ===");
    mutable_variables();
    println!("=== mutable variables end! ===");
}
