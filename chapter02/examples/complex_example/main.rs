mod calculator;
mod utils;

use calculator::Calculator;

fn main() {
    println!("=== 复杂示例 - 多文件项目 ===\n");

    let calc = Calculator::new();

    // 测试加法
    let result = calc.add(10, 20);
    println!("10 + 20 = {}", result);

    utils::print_separator();

    // 测试乘法
    let result = calc.multiply(5, 6);
    println!("5 × 6 = {}", result);

    utils::print_separator();

    // 使用格式化消息
    let msg = utils::format_message("计算完成！");
    println!("{}", msg);

    println!("\n=== 示例运行完成 ===");
}
