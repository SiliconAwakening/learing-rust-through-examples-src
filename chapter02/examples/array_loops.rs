fn array_loops() {
    let arr = [10, 20, 30, 40, 50];
    let mut sum = 0;

    // 方法1: 使用索引循环
    let len = arr.len();
    for i in 0..len {
        sum += arr[i];
        println!("添加 arr[{}] = {}, 当前总和: {}", i, arr[i], sum);
    }
    println!("数组总和: {}", sum);

    // 方法2: 直接遍历元素（更安全）
    let mut sum2 = 0;
    for &value in &arr {
        sum2 += value;
        println!("元素值: {}", value);
    }
    println!("重新计算的总和: {}", sum2);

    // 方法3: enumerate遍历
    for (i, &value) in arr.iter().enumerate() {
        println!("索引 {}: 值 {}", i, value);
    }
}

fn main() {
    println!("=== array loops example ===");
    array_loops();
    println!("=== array loops example end! ===");
}
