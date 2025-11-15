fn array_in_functions() {
    let arr = [1, 2, 3, 4, 5];

    // 传递数组引用
    let sum = sum_array(&arr);
    let max = max_array(&arr);

    println!("数组: {:?}", arr);
    println!("总和: {}, 最大值: {}", sum, max);

    // 修改数组（需要mut）
    let mut mut_arr = [10, 20, 30];
    modify_array(&mut mut_arr);
    println!("修改后: {:?}", mut_arr);

    // 返回数组
    let squared = square_array(&arr);
    println!("平方后: {:?}", squared);
}

// 计算数组总和
fn sum_array(arr: &[i32]) -> i32 {
    arr.iter().sum()
}

// 找最大值
fn max_array(arr: &[i32]) -> i32 {
    arr.iter().max().copied().unwrap_or(0)
}

// 修改数组元素
fn modify_array(arr: &mut [i32]) {
    for i in 0..arr.len() {
        arr[i] *= 2;
    }
}

// 返回平方数组
fn square_array(arr: &[i32]) -> Vec<i32> {
    arr.iter().map(|&x| x * x).collect()
}

fn main() {
    println!("=== array in functions example ===");
    array_in_functions();
    println!("=== array in functions example end! ===");
}
