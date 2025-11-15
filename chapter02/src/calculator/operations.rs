// calculator/operations.rs - 计算器操作定义

use std::fmt;

/// 计算操作类型
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Operation {
    /// 加法
    Add,
    /// 减法
    Subtract,
    /// 乘法
    Multiply,
    /// 除法
    Divide,
    /// 幂运算
    Power,
    /// 平方根
    SquareRoot,
    /// 正弦
    Sin,
    /// 余弦
    Cos,
    /// 正切
    Tan,
    /// 自然对数
    Ln,
    /// 对数（指定底数）
    Log,
    /// 阶乘
    Factorial,
    /// 绝对值
    Abs,
    /// 向上取整
    Ceil,
    /// 向下取整
    Floor,
    /// 四舍五入
    Round,
}

impl Operation {
    /// 获取操作的符号表示
    pub fn symbol(&self) -> &str {
        match self {
            Operation::Add => "+",
            Operation::Subtract => "-",
            Operation::Multiply => "*",
            Operation::Divide => "/",
            Operation::Power => "^",
            Operation::SquareRoot => "√",
            Operation::Sin => "sin",
            Operation::Cos => "cos",
            Operation::Tan => "tan",
            Operation::Ln => "ln",
            Operation::Log => "log",
            Operation::Factorial => "!",
            Operation::Abs => "abs",
            Operation::Ceil => "ceil",
            Operation::Floor => "floor",
            Operation::Round => "round",
        }
    }

    /// 获取操作的优先级
    pub fn precedence(&self) -> u8 {
        match self {
            Operation::Add | Operation::Subtract => 1,
            Operation::Multiply | Operation::Divide => 2,
            Operation::Power => 3,
            _ => 4, // 函数操作具有最高优先级
        }
    }

    /// 判断是否为二元操作
    pub fn is_binary(&self) -> bool {
        matches!(
            self,
            Operation::Add
                | Operation::Subtract
                | Operation::Multiply
                | Operation::Divide
                | Operation::Power
                | Operation::Log
        )
    }

    /// 判断是否为一元操作
    pub fn is_unary(&self) -> bool {
        matches!(
            self,
            Operation::SquareRoot
                | Operation::Sin
                | Operation::Cos
                | Operation::Tan
                | Operation::Ln
                | Operation::Factorial
                | Operation::Abs
                | Operation::Ceil
                | Operation::Floor
                | Operation::Round
        )
    }

    /// 从字符串解析操作
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "+" => Some(Operation::Add),
            "-" => Some(Operation::Subtract),
            "*" | "×" => Some(Operation::Multiply),
            "/" | "÷" => Some(Operation::Divide),
            "^" | "**" => Some(Operation::Power),
            "sqrt" | "√" => Some(Operation::SquareRoot),
            "sin" => Some(Operation::Sin),
            "cos" => Some(Operation::Cos),
            "tan" => Some(Operation::Tan),
            "ln" => Some(Operation::Ln),
            "log" => Some(Operation::Log),
            "!" => Some(Operation::Factorial),
            "abs" => Some(Operation::Abs),
            "ceil" => Some(Operation::Ceil),
            "floor" => Some(Operation::Floor),
            "round" => Some(Operation::Round),
            _ => None,
        }
    }
}

impl fmt::Display for Operation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.symbol())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_operation_symbol() {
        assert_eq!(Operation::Add.symbol(), "+");
        assert_eq!(Operation::Multiply.symbol(), "*");
        assert_eq!(Operation::SquareRoot.symbol(), "√");
    }

    #[test]
    fn test_operation_precedence() {
        assert!(Operation::Multiply.precedence() > Operation::Add.precedence());
        assert!(Operation::Power.precedence() > Operation::Multiply.precedence());
    }

    #[test]
    fn test_operation_type() {
        assert!(Operation::Add.is_binary());
        assert!(!Operation::Add.is_unary());
        assert!(Operation::SquareRoot.is_unary());
        assert!(!Operation::SquareRoot.is_binary());
    }

    #[test]
    fn test_from_str() {
        assert_eq!(Operation::from_str("+"), Some(Operation::Add));
        assert_eq!(Operation::from_str("sqrt"), Some(Operation::SquareRoot));
        assert_eq!(Operation::from_str("×"), Some(Operation::Multiply));
        assert_eq!(Operation::from_str("invalid"), None);
    }
}
