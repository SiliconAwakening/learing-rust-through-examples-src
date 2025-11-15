fn float_types() {
    let f32_val: f32 = 3.141592653589793; // 32位浮点
    let f64_val: f64 = 3.141592653589793; // 64位浮点（默认）

    // 特殊值
    let infinity = f32::INFINITY;
    let neg_infinity = f32::NEG_INFINITY;
    let not_a_number = f32::NAN;

    println!("f32: {}", f32_val);
    println!("f64: {}", f64_val);
    println!("无穷大: {}", infinity);
    println!("负无穷大: {}", neg_infinity);
    println!("非数字: {}", not_a_number);

    // 数学运算
    let result = f32::sqrt(2.0);
    println!("√2 = {}", result);

    // 比较运算
    let x: f64 = 1.0;
    let y: f64 = 0.1 + 0.1 + 0.1 + 0.1 + 0.1;
    println!("x == y: {}", x == y); // 避免直接比较浮点数
    println!("(x - y).abs() < 1e-10: {}", (x - y).abs() < 1e-10);
}
fn main() {
    println!("=== float types example ===");
    float_types();
    println!("=== float types example end! ===");
}
