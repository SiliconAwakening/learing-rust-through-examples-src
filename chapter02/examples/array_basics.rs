fn array_basics() {
    // 数组声明和初始化
    let numbers: [i32; 5] = [1, 2, 3, 4, 5];
    let floats = [3.14, 2.71, 1.41, 1.73]; // 类型推导
    let chars = ['R', 'u', 's', 't']; // 字符数组

    // 访问数组元素
    let first = numbers[0];
    let last = numbers[4];
    println!("第一个元素: {}, 最后一个元素: {}", first, last);

    // 数组长度
    println!("numbers数组长度: {}", numbers.len());

    // 初始化相同值的数组
    let repeated = [0; 10]; // 长度为10的数组，所有元素都是0
    println!("重复值数组长度: {}", repeated.len());

    // 遍历数组
    for (index, &value) in numbers.iter().enumerate() {
        println!("numbers[{}] = {}", index, value);
    }
}

fn main() {
    println!("=== array basics example ===");
    array_basics();
    println!("=== array basics example end! ===");
}
