// 多个参数
fn calculate_area(length: f64, width: f64) -> f64 {
    length * width
}

// 可变参数数量
fn print_values(values: &[i32]) {
    for value in values {
        println!("值: {}", value);
    }
}

// 元组返回
fn get_coordinates() -> (i32, i32) {
    (10, 20)
}

// 命名返回结构
#[derive(Debug)]
struct Rectangle {
    width: f64,
    height: f64,
}

fn create_rectangle(width: f64, height: f64) -> Rectangle {
    Rectangle { width, height }
}

fn calculate_rectangle_area(rect: &Rectangle) -> f64 {
    rect.width * rect.height
}

fn function_parameters() {
    let area = calculate_area(5.0, 3.0);
    println!("矩形面积: {}", area);

    let values = vec![1, 2, 3, 4, 5];
    print_values(&values);

    let (x, y) = get_coordinates();
    println!("坐标: ({}, {})", x, y);

    let rectangle = create_rectangle(4.0, 6.0);
    let rect_area = calculate_rectangle_area(&rectangle);
    println!("矩形面积: {}", rect_area);
}

fn main() {
    println!("=== function parameters example ===");
    function_parameters();
    println!("=== function parameters example end! ===");
}
