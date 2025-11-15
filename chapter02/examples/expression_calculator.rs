// examples/expression_calculator.rs - 表达式计算器示例

use chapter02::calculator::Calculator;
use std::io::{self, Write};

fn main() {
    println!("=== 表达式计算器 ===");
    println!("支持的操作符: +, -, *, /, ^ (幂)");
    println!("支持的函数: sin, cos, tan, sqrt, ln, abs, ceil, floor, round");
    println!("支持括号改变优先级");
    println!("输入 'quit' 或 'exit' 退出");
    println!();

    let calc = Calculator::new();

    // 演示一些示例
    println!("示例表达式:");
    let examples = vec![
        "2 + 3",
        "10 - 4 * 2",
        "(10 - 4) * 2",
        "2 ^ 3",
        "2 ^ 3 ^ 2",
        "sqrt(16)",
        "sin(0)",
        "2 + sqrt(16) * 3",
        "(2 + 3) * (4 + 5)",
        "-5 + 10",
        "abs(-42)",
        "ceil(3.14)",
        "floor(3.99)",
    ];

    for expr in &examples {
        match calc.evaluate_expression(expr) {
            Ok(result) => println!("  {} = {}", expr, result),
            Err(e) => println!("  {} => 错误: {}", expr, e),
        }
    }

    println!();
    println!("现在可以输入你自己的表达式:");
    println!();

    // 交互式计算器
    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("读取输入失败");

        let input = input.trim();

        if input.is_empty() {
            continue;
        }

        if input == "quit" || input == "exit" {
            println!("再见！");
            break;
        }

        match calc.evaluate_expression(input) {
            Ok(result) => println!("= {}", result),
            Err(e) => println!("错误: {}", e),
        }
    }
}
