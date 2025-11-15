// utils/error.rs - 错误类型定义

use std::fmt;

/// 计算器错误类型
#[derive(Debug, Clone, PartialEq)]
pub enum Error {
    /// 除零错误
    DivisionByZero,

    /// 负数开平方根
    NegativeSquareRoot,

    /// 无效的对数运算
    InvalidLogarithm,

    /// 阶乘参数无效
    InvalidFactorialArgument,

    /// 阶乘值过大
    FactorialTooLarge,

    /// 空数据集
    EmptyDataSet,

    /// 无效数字
    InvalidNumber(String),

    /// 无效字符
    InvalidCharacter(char),

    /// 括号不匹配
    MismatchedParen,

    /// 无效表达式
    InvalidExpression,

    /// 解析错误
    ParseError(String),

    /// 操作数不足
    InsufficientOperands,

    /// 无效操作符
    InvalidOperator,

    /// 未定义变量
    UndefinedVariable(String),

    /// 未定义函数
    UndefinedFunction(String),

    /// 函数参数数量错误
    InvalidArgumentCount {
        function: String,
        expected: usize,
        actual: usize,
    },

    /// IO 错误
    IoError(String),

    /// 其他错误
    Other(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::DivisionByZero => write!(f, "除零错误"),
            Error::NegativeSquareRoot => write!(f, "负数不能开平方根"),
            Error::InvalidLogarithm => {
                write!(f, "无效的对数运算: 底数必须 > 0 且 ≠ 1, 真数必须 > 0")
            }
            Error::InvalidFactorialArgument => {
                write!(f, "阶乘参数无效: 必须是非负整数")
            }
            Error::FactorialTooLarge => write!(f, "阶乘值过大: n > 20"),
            Error::EmptyDataSet => write!(f, "空数据集"),
            Error::InvalidNumber(s) => write!(f, "无效数字: {}", s),
            Error::InvalidCharacter(c) => write!(f, "无效字符: {}", c),
            Error::MismatchedParen => write!(f, "括号不匹配"),
            Error::InvalidExpression => write!(f, "无效表达式"),
            Error::ParseError(s) => write!(f, "解析错误: {}", s),
            Error::InsufficientOperands => write!(f, "操作数不足"),
            Error::InvalidOperator => write!(f, "无效操作符"),
            Error::UndefinedVariable(s) => write!(f, "未定义变量: {}", s),
            Error::UndefinedFunction(s) => write!(f, "未定义函数: {}", s),
            Error::InvalidArgumentCount {
                function,
                expected,
                actual,
            } => write!(
                f,
                "函数 {} 参数数量错误: 期望 {}, 实际 {}",
                function, expected, actual
            ),
            Error::IoError(s) => write!(f, "IO 错误: {}", s),
            Error::Other(s) => write!(f, "错误: {}", s),
        }
    }
}

impl std::error::Error for Error {}

// 从 std::io::Error 转换
impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::IoError(err.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        let err = Error::DivisionByZero;
        assert_eq!(err.to_string(), "除零错误");

        let err = Error::InvalidNumber("abc".to_string());
        assert_eq!(err.to_string(), "无效数字: abc");
    }

    #[test]
    fn test_error_clone() {
        let err1 = Error::DivisionByZero;
        let err2 = err1.clone();
        assert_eq!(err1, err2);
    }
}
