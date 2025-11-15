fn array_operations() {
    let mut numbers = [64, 34, 25, 12, 22, 11, 90];

    println!("原始数组: {:?}", numbers);

    // 查找最大值和最小值
    let max = numbers.iter().max().unwrap();
    let min = numbers.iter().min().unwrap();
    println!("最大值: {}, 最小值: {}", max, min);

    // 计算数组总和和平均值
    let sum: i32 = numbers.iter().sum();
    let average = sum as f64 / numbers.len() as f64;
    println!("总和: {}, 平均值: {:.2}", sum, average);

    // 过滤和变换
    let even_numbers: Vec<_> = numbers.iter().filter(|&&x| x % 2 == 0).copied().collect();
    println!("偶数: {:?}", even_numbers);

    let squared: Vec<_> = numbers.iter().map(|&x| x * x).collect();
    println!("平方: {:?}", squared);

    // 检查是否包含某个值
    let contains_25 = numbers.contains(&25);
    let position = numbers.iter().position(|&x| x == 25);
    println!("包含25: {}, 位置: {:?}", contains_25, position);

    // 排序
    let mut sorted = numbers;
    sorted.sort();
    println!("排序后: {:?}", sorted);
}

fn main() {
    println!("=== array operations example ===");
    array_operations();
    println!("=== array operations example end! ===");
}
