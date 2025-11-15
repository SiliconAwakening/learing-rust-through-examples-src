fn array_bounds_checking() {
    let arr = [10, 20, 30];

    // 安全的访问
    if let Some(&value) = arr.get(1) {
        println!("arr[1] = {}", value);
    }

    // 检查是否越界
    match arr.get(5) {
        Some(value) => println!("arr[5] = {}", value),
        None => println!("数组越界! 最大索引: {}", arr.len() - 1),
    }

    // 数组切片（引用数组的一部分）
    let slice = &arr[0..2]; // 包含索引0到1
    println!("切片内容: {:?}", slice);

    let slice_to_end = &arr[1..]; // 从索引1到末尾
    println!("从索引1开始的切片: {:?}", slice_to_end);

    let slice_from_start = &arr[..2]; // 从开头到索引2（不包含2）
    println!("从开头到索引2的切片: {:?}", slice_from_start);

    let full_slice = &arr[..]; // 整个数组的切片
    println!("完整切片: {:?}", full_slice);
}

fn main() {
    println!("=== array bounds checking example ===");
    array_bounds_checking();
    println!("=== array bounds checking example end! ===");
}
