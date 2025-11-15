fn multidimensional_arrays() {
    // 二维数组
    let matrix: [[i32; 3]; 2] = [[1, 2, 3], [4, 5, 6]];

    println!("矩阵内容:");
    for (i, row) in matrix.iter().enumerate() {
        for (j, &value) in row.iter().enumerate() {
            print!("matrix[{}][{}] = {}  ", i, j, value);
        }
        println!();
    }

    // 访问二维数组元素
    let element = matrix[1][2]; // 第二行第三列的值: 6
    println!("matrix[1][2] = {}", element);

    // 三维数组示例
    let three_d: [[[i32; 2]; 2]; 2] = [[[1, 2], [3, 4]], [[5, 6], [7, 8]]];

    println!("三维数组内容:");
    for (i, depth) in three_d.iter().enumerate() {
        for (j, row) in depth.iter().enumerate() {
            for (k, &value) in row.iter().enumerate() {
                print!("[{}][{}][{}] = {}  ", i, j, k, value);
            }
            println!();
        }
    }
}

fn main() {
    println!("=== multidimensional arrays example ===");
    multidimensional_arrays();
    println!("=== multidimensional arrays example end! ===");
}
