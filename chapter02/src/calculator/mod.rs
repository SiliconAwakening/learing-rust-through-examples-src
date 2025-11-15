// calculator/mod.rs - 计算器模块

pub mod evaluator;
pub mod operations;
pub mod parser;

use crate::data::Statistics;
use crate::utils::Error;

// 重新导出
pub use operations::Operation;

/// 科学计算器
pub struct Calculator {
    precision: u8,
}

impl Calculator {
    /// 创建新的计算器实例
    pub fn new() -> Self {
        Self { precision: 10 }
    }

    /// 设置计算精度
    pub fn set_precision(&mut self, precision: u8) {
        self.precision = precision;
    }

    // ===== 基础运算 =====

    /// 加法运算
    pub fn add(&self, a: f64, b: f64) -> Result<f64, Error> {
        Ok(a + b)
    }

    /// 减法运算
    pub fn subtract(&self, a: f64, b: f64) -> Result<f64, Error> {
        Ok(a - b)
    }

    /// 乘法运算
    pub fn multiply(&self, a: f64, b: f64) -> Result<f64, Error> {
        Ok(a * b)
    }

    /// 除法运算
    pub fn divide(&self, a: f64, b: f64) -> Result<f64, Error> {
        if b == 0.0 {
            return Err(Error::DivisionByZero);
        }
        Ok(a / b)
    }

    /// 幂运算
    pub fn power(&self, base: f64, exponent: f64) -> Result<f64, Error> {
        Ok(base.powf(exponent))
    }

    // ===== 科学运算 =====

    /// 平方根
    pub fn sqrt(&self, value: f64) -> Result<f64, Error> {
        if value < 0.0 {
            return Err(Error::NegativeSquareRoot);
        }
        Ok(value.sqrt())
    }

    /// 正弦函数
    pub fn sin(&self, angle: f64) -> Result<f64, Error> {
        Ok(angle.sin())
    }

    /// 余弦函数
    pub fn cos(&self, angle: f64) -> Result<f64, Error> {
        Ok(angle.cos())
    }

    /// 正切函数
    pub fn tan(&self, angle: f64) -> Result<f64, Error> {
        Ok(angle.tan())
    }

    /// 自然对数
    pub fn ln(&self, value: f64) -> Result<f64, Error> {
        if value <= 0.0 {
            return Err(Error::InvalidLogarithm);
        }
        Ok(value.ln())
    }

    /// 对数（指定底数）
    pub fn log(&self, value: f64, base: f64) -> Result<f64, Error> {
        if value <= 0.0 || base <= 0.0 || base == 1.0 {
            return Err(Error::InvalidLogarithm);
        }
        Ok(value.log(base))
    }

    /// 阶乘
    pub fn factorial(&self, n: u64) -> Result<f64, Error> {
        if n > 20 {
            return Err(Error::FactorialTooLarge);
        }
        let result = (1..=n).product::<u64>() as f64;
        Ok(result)
    }

    /// 绝对值
    pub fn abs(&self, value: f64) -> f64 {
        value.abs()
    }

    // ===== 表达式求值 =====

    /// 求值表达式字符串
    ///
    /// # 示例
    ///
    /// ```
    /// let calc = Calculator::new();
    /// assert_eq!(calc.evaluate_expression("2 + 3 * 4").unwrap(), 14.0);
    /// assert_eq!(calc.evaluate_expression("(2 + 3) * 4").unwrap(), 20.0);
    /// assert_eq!(calc.evaluate_expression("sin(0)").unwrap(), 0.0);
    /// assert_eq!(calc.evaluate_expression("sqrt(16)").unwrap(), 4.0);
    /// ```
    pub fn evaluate_expression(&self, expression: &str) -> Result<f64, Error> {
        // 1. 词法分析：将字符串转为 token 流
        let parser = parser::ExpressionParser::new();
        let tokens = parser.tokenize(expression)?;

        // 2. 语法分析：将 token 流转为抽象语法树 (AST)
        let ast = parser.parse(tokens)?;

        // 3. 求值：遍历 AST 计算结果
        let evaluator = evaluator::ExpressionEvaluator::new();
        evaluator.evaluate(&ast)
    }

    // ===== 统计计算 =====

    /// 计算数据集的统计信息
    pub fn calculate_statistics(&self, data: &[f64]) -> Result<Statistics, Error> {
        if data.is_empty() {
            return Err(Error::EmptyDataSet);
        }

        let n = data.len() as f64;
        let sum: f64 = data.iter().sum();
        let mean = sum / n;

        // 计算方差
        let variance: f64 = data.iter().map(|&x| (x - mean).powi(2)).sum::<f64>() / n;
        let std_dev = variance.sqrt();

        // 计算中位数
        let mut sorted_data = data.to_vec();
        sorted_data.sort_by(|a, b| a.partial_cmp(b).unwrap());
        let median = if sorted_data.len() % 2 == 0 {
            let mid = sorted_data.len() / 2;
            (sorted_data[mid - 1] + sorted_data[mid]) / 2.0
        } else {
            sorted_data[sorted_data.len() / 2]
        };

        // 找最小值和最大值
        let min = *sorted_data.first().unwrap();
        let max = *sorted_data.last().unwrap();

        // 计算众数
        let mut frequency = std::collections::HashMap::new();
        for &value in data {
            *frequency.entry(value.to_bits()).or_insert(0) += 1;
        }
        let max_count = *frequency.values().max().unwrap_or(&0);
        let mode: Vec<f64> = frequency
            .iter()
            .filter(|(_, &count)| count == max_count)
            .map(|(&bits, _)| f64::from_bits(bits))
            .collect();

        Ok(Statistics {
            count: data.len(),
            mean,
            median,
            mode,
            variance,
            std_dev,
            min,
            max,
            sum,
        })
    }
}

impl Default for Calculator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_operations() {
        let calc = Calculator::new();
        assert_eq!(calc.add(2.0, 3.0).unwrap(), 5.0);
        assert_eq!(calc.subtract(5.0, 3.0).unwrap(), 2.0);
        assert_eq!(calc.multiply(4.0, 5.0).unwrap(), 20.0);
        assert_eq!(calc.divide(10.0, 2.0).unwrap(), 5.0);
    }

    #[test]
    fn test_division_by_zero() {
        let calc = Calculator::new();
        assert!(calc.divide(10.0, 0.0).is_err());
    }

    #[test]
    fn test_sqrt() {
        let calc = Calculator::new();
        assert_eq!(calc.sqrt(16.0).unwrap(), 4.0);
        assert!(calc.sqrt(-1.0).is_err());
    }

    #[test]
    fn test_factorial() {
        let calc = Calculator::new();
        assert_eq!(calc.factorial(5).unwrap(), 120.0);
        assert!(calc.factorial(21).is_err());
    }

    #[test]
    fn test_evaluate_simple_expression() {
        let calc = Calculator::new();
        assert_eq!(calc.evaluate_expression("2 + 3").unwrap(), 5.0);
        assert_eq!(calc.evaluate_expression("10 - 4").unwrap(), 6.0);
        assert_eq!(calc.evaluate_expression("3 * 7").unwrap(), 21.0);
        assert_eq!(calc.evaluate_expression("20 / 4").unwrap(), 5.0);
    }

    #[test]
    fn test_evaluate_operator_precedence() {
        let calc = Calculator::new();
        // 2 + 3 * 4 = 2 + 12 = 14
        assert_eq!(calc.evaluate_expression("2 + 3 * 4").unwrap(), 14.0);
        // 10 - 2 * 3 = 10 - 6 = 4
        assert_eq!(calc.evaluate_expression("10 - 2 * 3").unwrap(), 4.0);
    }

    #[test]
    fn test_evaluate_parentheses() {
        let calc = Calculator::new();
        // (2 + 3) * 4 = 5 * 4 = 20
        assert_eq!(calc.evaluate_expression("(2 + 3) * 4").unwrap(), 20.0);
        // 2 * (3 + 4) = 2 * 7 = 14
        assert_eq!(calc.evaluate_expression("2 * (3 + 4)").unwrap(), 14.0);
    }

    #[test]
    fn test_evaluate_functions() {
        let calc = Calculator::new();
        assert_eq!(calc.evaluate_expression("sin(0)").unwrap(), 0.0);
        assert_eq!(calc.evaluate_expression("sqrt(16)").unwrap(), 4.0);
        assert_eq!(calc.evaluate_expression("abs(-5)").unwrap(), 5.0);
    }

    #[test]
    fn test_evaluate_complex_expression() {
        let calc = Calculator::new();
        // 2 + sqrt(16) * 3 = 2 + 4 * 3 = 2 + 12 = 14
        assert_eq!(calc.evaluate_expression("2 + sqrt(16) * 3").unwrap(), 14.0);
    }

    #[test]
    fn test_evaluate_power() {
        let calc = Calculator::new();
        // 2 ^ 3 = 8
        assert_eq!(calc.evaluate_expression("2 ^ 3").unwrap(), 8.0);
        // 2 ^ 3 ^ 2 = 2 ^ 9 = 512 (右结合)
        assert_eq!(calc.evaluate_expression("2 ^ 3 ^ 2").unwrap(), 512.0);
    }

    #[test]
    fn test_evaluate_unary_minus() {
        let calc = Calculator::new();
        assert_eq!(calc.evaluate_expression("-5").unwrap(), -5.0);
        assert_eq!(calc.evaluate_expression("-(2 + 3)").unwrap(), -5.0);
        assert_eq!(calc.evaluate_expression("10 + -5").unwrap(), 5.0);
    }
}
