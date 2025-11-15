// calculator/evaluator.rs - 表达式求值器（简化版）

use super::parser::AstNode;
use crate::utils::Error;

/// 表达式求值器
pub struct ExpressionEvaluator {
    // 预留字段供将来扩展
    _strict_mode: bool,
}

impl ExpressionEvaluator {
    /// 创建新的求值器
    pub fn new() -> Self {
        Self {
            _strict_mode: false,
        }
    }

    /// 对抽象语法树进行求值
    pub fn evaluate(&self, ast: &AstNode) -> Result<f64, Error> {
        match ast {
            AstNode::Number(n) => Ok(*n),

            AstNode::BinaryOp { op, left, right } => {
                let left_val = self.evaluate(left)?;
                let right_val = self.evaluate(right)?;

                match op {
                    '+' => Ok(left_val + right_val),
                    '-' => Ok(left_val - right_val),
                    '*' => Ok(left_val * right_val),
                    '/' => {
                        if right_val == 0.0 {
                            Err(Error::DivisionByZero)
                        } else {
                            Ok(left_val / right_val)
                        }
                    }
                    '^' => Ok(left_val.powf(right_val)),
                    _ => Err(Error::InvalidOperator),
                }
            }

            AstNode::UnaryOp { op, operand } => {
                let val = self.evaluate(operand)?;

                match op {
                    '-' => Ok(-val),
                    '+' => Ok(val),
                    _ => Err(Error::InvalidOperator),
                }
            }

            AstNode::FunctionCall { name, args } => self.evaluate_function(name, args),
        }
    }

    /// 求值函数调用
    fn evaluate_function(&self, name: &str, args: &[AstNode]) -> Result<f64, Error> {
        match name.to_lowercase().as_str() {
            "sin" => {
                if args.len() != 1 {
                    return Err(Error::InvalidArgumentCount {
                        function: name.to_string(),
                        expected: 1,
                        actual: args.len(),
                    });
                }
                let arg = self.evaluate(&args[0])?;
                Ok(arg.sin())
            }

            "cos" => {
                if args.len() != 1 {
                    return Err(Error::InvalidArgumentCount {
                        function: name.to_string(),
                        expected: 1,
                        actual: args.len(),
                    });
                }
                let arg = self.evaluate(&args[0])?;
                Ok(arg.cos())
            }

            "tan" => {
                if args.len() != 1 {
                    return Err(Error::InvalidArgumentCount {
                        function: name.to_string(),
                        expected: 1,
                        actual: args.len(),
                    });
                }
                let arg = self.evaluate(&args[0])?;
                Ok(arg.tan())
            }

            "sqrt" => {
                if args.len() != 1 {
                    return Err(Error::InvalidArgumentCount {
                        function: name.to_string(),
                        expected: 1,
                        actual: args.len(),
                    });
                }
                let arg = self.evaluate(&args[0])?;
                if arg < 0.0 {
                    return Err(Error::NegativeSquareRoot);
                }
                Ok(arg.sqrt())
            }

            "ln" => {
                if args.len() != 1 {
                    return Err(Error::InvalidArgumentCount {
                        function: name.to_string(),
                        expected: 1,
                        actual: args.len(),
                    });
                }
                let arg = self.evaluate(&args[0])?;
                if arg <= 0.0 {
                    return Err(Error::InvalidLogarithm);
                }
                Ok(arg.ln())
            }

            "log" => {
                if args.len() != 2 {
                    return Err(Error::InvalidArgumentCount {
                        function: name.to_string(),
                        expected: 2,
                        actual: args.len(),
                    });
                }
                let value = self.evaluate(&args[0])?;
                let base = self.evaluate(&args[1])?;

                if value <= 0.0 || base <= 0.0 || base == 1.0 {
                    return Err(Error::InvalidLogarithm);
                }
                Ok(value.log(base))
            }

            "abs" => {
                if args.len() != 1 {
                    return Err(Error::InvalidArgumentCount {
                        function: name.to_string(),
                        expected: 1,
                        actual: args.len(),
                    });
                }
                let arg = self.evaluate(&args[0])?;
                Ok(arg.abs())
            }

            "ceil" => {
                if args.len() != 1 {
                    return Err(Error::InvalidArgumentCount {
                        function: name.to_string(),
                        expected: 1,
                        actual: args.len(),
                    });
                }
                let arg = self.evaluate(&args[0])?;
                Ok(arg.ceil())
            }

            "floor" => {
                if args.len() != 1 {
                    return Err(Error::InvalidArgumentCount {
                        function: name.to_string(),
                        expected: 1,
                        actual: args.len(),
                    });
                }
                let arg = self.evaluate(&args[0])?;
                Ok(arg.floor())
            }

            "round" => {
                if args.len() != 1 {
                    return Err(Error::InvalidArgumentCount {
                        function: name.to_string(),
                        expected: 1,
                        actual: args.len(),
                    });
                }
                let arg = self.evaluate(&args[0])?;
                Ok(arg.round())
            }

            _ => Err(Error::UndefinedFunction(name.to_string())),
        }
    }
}

impl Default for ExpressionEvaluator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_evaluate_number() {
        let evaluator = ExpressionEvaluator::new();
        let ast = AstNode::Number(42.0);
        assert_eq!(evaluator.evaluate(&ast).unwrap(), 42.0);
    }

    #[test]
    fn test_evaluate_addition() {
        let evaluator = ExpressionEvaluator::new();
        let ast = AstNode::BinaryOp {
            op: '+',
            left: Box::new(AstNode::Number(10.0)),
            right: Box::new(AstNode::Number(5.0)),
        };
        assert_eq!(evaluator.evaluate(&ast).unwrap(), 15.0);
    }

    #[test]
    fn test_evaluate_multiplication() {
        let evaluator = ExpressionEvaluator::new();
        let ast = AstNode::BinaryOp {
            op: '*',
            left: Box::new(AstNode::Number(6.0)),
            right: Box::new(AstNode::Number(7.0)),
        };
        assert_eq!(evaluator.evaluate(&ast).unwrap(), 42.0);
    }

    #[test]
    fn test_evaluate_division_by_zero() {
        let evaluator = ExpressionEvaluator::new();
        let ast = AstNode::BinaryOp {
            op: '/',
            left: Box::new(AstNode::Number(10.0)),
            right: Box::new(AstNode::Number(0.0)),
        };
        assert!(evaluator.evaluate(&ast).is_err());
    }

    #[test]
    fn test_evaluate_function_sin() {
        let evaluator = ExpressionEvaluator::new();
        let ast = AstNode::FunctionCall {
            name: "sin".to_string(),
            args: vec![AstNode::Number(0.0)],
        };
        assert_eq!(evaluator.evaluate(&ast).unwrap(), 0.0);
    }

    #[test]
    fn test_evaluate_function_sqrt() {
        let evaluator = ExpressionEvaluator::new();
        let ast = AstNode::FunctionCall {
            name: "sqrt".to_string(),
            args: vec![AstNode::Number(16.0)],
        };
        assert_eq!(evaluator.evaluate(&ast).unwrap(), 4.0);
    }

    #[test]
    fn test_evaluate_function_sqrt_negative() {
        let evaluator = ExpressionEvaluator::new();
        let ast = AstNode::FunctionCall {
            name: "sqrt".to_string(),
            args: vec![AstNode::Number(-1.0)],
        };
        assert!(evaluator.evaluate(&ast).is_err());
    }

    #[test]
    fn test_nested_operations() {
        let evaluator = ExpressionEvaluator::new();
        // (2 + 3) * 4 = 20
        let ast = AstNode::BinaryOp {
            op: '*',
            left: Box::new(AstNode::BinaryOp {
                op: '+',
                left: Box::new(AstNode::Number(2.0)),
                right: Box::new(AstNode::Number(3.0)),
            }),
            right: Box::new(AstNode::Number(4.0)),
        };
        assert_eq!(evaluator.evaluate(&ast).unwrap(), 20.0);
    }
}
