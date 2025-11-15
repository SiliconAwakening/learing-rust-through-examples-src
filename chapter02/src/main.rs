// src/main.rs
mod calculator;
mod data;
mod history;
mod utils;

use calculator::{Calculator, Operation};
use data::Statistics;
use history::HistoryManager;
use utils::Error;

fn main() -> Result<(), Error> {
    println!("=== 科学计算器 v1.0 ===");

    let mut calculator = Calculator::new();
    let mut history = HistoryManager::new();

    // 示例计算
    run_example_calculations(&mut calculator, &mut history)?;

    Ok(())
}

fn run_example_calculations(
    calc: &mut Calculator,
    history: &mut HistoryManager,
) -> Result<(), Error> {
    // 基础运算
    let result1 = calc.add(10.0, 5.0)?;
    println!("10 + 5 = {}", result1);
    history.add_record("10 + 5", result1);

    let result2 = calc.multiply(result1, 2.0)?;
    println!("({}) × 2 = {}", result1, result2);
    history.add_record("10 + 5 * 2", result2);

    // 科学运算
    let result3 = calc.sqrt(16.0)?;
    println!("√16 = {}", result3);
    history.add_record("√16", result3);

    let result4 = calc.sin(30.0_f64.to_radians())?;
    println!("sin(30°) = {}", result4);
    history.add_record("sin(30°)", result4);

    // 表达式求值
    let expr_result = calc.evaluate_expression("(10 + 5) * 2 - √16")?;
    println!("(10 + 5) * 2 - √16 = {}", expr_result);
    history.add_record("(10 + 5) * 2 - √16", expr_result);

    // 统计计算
    let data = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0];
    let stats = calc.calculate_statistics(&data)?;
    println!("数据集统计: {:?}", stats);

    history.display();

    Ok(())
}
