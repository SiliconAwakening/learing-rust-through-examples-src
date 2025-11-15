// calculator/parser.rs - 表达式解析器（简化版）

use crate::utils::Error;

/// 词法单元类型
#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    /// 数字
    Number(f64),
    /// 操作符
    Operator(char),
    /// 左括号
    LeftParen,
    /// 右括号
    RightParen,
    /// 函数名
    Function(String),
    /// 常量（如 π, e）
    Constant(String),
    /// 前缀运算符（如 √）
    PrefixOp(String),
}

/// 表达式解析器
pub struct ExpressionParser {
    // 预留字段供将来扩展
    _allow_functions: bool,
}

impl ExpressionParser {
    /// 创建新的解析器
    pub fn new() -> Self {
        Self {
            _allow_functions: true,
        }
    }

    /// 词法分析：将字符串转换为 Token 序列
    pub fn tokenize(&self, expression: &str) -> Result<Vec<Token>, Error> {
        let mut tokens = Vec::new();
        let chars: Vec<char> = expression.chars().collect();
        let mut i = 0;

        while i < chars.len() {
            let ch = chars[i];

            // 跳过空白字符
            if ch.is_whitespace() {
                i += 1;
                continue;
            }

            // 处理 Unicode 数学符号
            match ch {
                // 平方根符号 √
                '√' => {
                    tokens.push(Token::PrefixOp("sqrt".to_string()));
                    i += 1;
                    continue;
                }
                // 圆周率 π
                'π' => {
                    tokens.push(Token::Constant("pi".to_string()));
                    i += 1;
                    continue;
                }
                // 自然常数 e
                'ℯ' | 'ⅇ' => {
                    tokens.push(Token::Constant("e".to_string()));
                    i += 1;
                    continue;
                }
                // 乘法 ×
                '×' => {
                    tokens.push(Token::Operator('*'));
                    i += 1;
                    continue;
                }
                // 除法 ÷
                '÷' => {
                    tokens.push(Token::Operator('/'));
                    i += 1;
                    continue;
                }
                // 上标平方 ²
                '²' => {
                    tokens.push(Token::Operator('^'));
                    tokens.push(Token::Number(2.0));
                    i += 1;
                    continue;
                }
                // 上标立方 ³
                '³' => {
                    tokens.push(Token::Operator('^'));
                    tokens.push(Token::Number(3.0));
                    i += 1;
                    continue;
                }
                _ => {}
            }

            // 解析数字
            if ch.is_ascii_digit() || ch == '.' {
                let start = i;
                while i < chars.len() && (chars[i].is_ascii_digit() || chars[i] == '.') {
                    i += 1;
                }
                let num_str: String = chars[start..i].iter().collect();
                let num = num_str
                    .parse::<f64>()
                    .map_err(|_| Error::InvalidNumber(num_str.clone()))?;
                tokens.push(Token::Number(num));
                continue;
            }

            // 解析操作符
            if "+-*/^".contains(ch) {
                tokens.push(Token::Operator(ch));
                i += 1;
                continue;
            }

            // 解析括号
            if ch == '(' {
                tokens.push(Token::LeftParen);
                i += 1;
                continue;
            }
            if ch == ')' {
                tokens.push(Token::RightParen);
                i += 1;
                continue;
            }

            // 解析函数名（字母开头）
            if ch.is_alphabetic() {
                let start = i;
                while i < chars.len() && chars[i].is_alphabetic() {
                    i += 1;
                }
                let func_name: String = chars[start..i].iter().collect();
                tokens.push(Token::Function(func_name));
                continue;
            }

            // 未知字符
            return Err(Error::InvalidCharacter(ch));
        }

        Ok(tokens)
    }

    /// 语法分析：将 Token 序列转换为抽象语法树
    pub fn parse(&self, tokens: Vec<Token>) -> Result<AstNode, Error> {
        if tokens.is_empty() {
            return Err(Error::ParseError("空表达式".to_string()));
        }

        let mut parser = Parser {
            tokens,
            position: 0,
        };

        parser.parse_expression()
    }
}

/// 内部解析器状态
struct Parser {
    tokens: Vec<Token>,
    position: usize,
}

impl Parser {
    /// 获取当前 token
    fn current(&self) -> Option<&Token> {
        self.tokens.get(self.position)
    }

    /// 前进到下一个 token
    fn advance(&mut self) {
        self.position += 1;
    }

    /// 解析表达式（最低优先级）
    fn parse_expression(&mut self) -> Result<AstNode, Error> {
        self.parse_additive()
    }

    /// 解析加减法（优先级 1）
    fn parse_additive(&mut self) -> Result<AstNode, Error> {
        let mut left = self.parse_multiplicative()?;

        while let Some(Token::Operator(op)) = self.current() {
            if *op != '+' && *op != '-' {
                break;
            }
            let op = *op;
            self.advance();
            let right = self.parse_multiplicative()?;
            left = AstNode::BinaryOp {
                op,
                left: Box::new(left),
                right: Box::new(right),
            };
        }

        Ok(left)
    }

    /// 解析乘除法（优先级 2）
    fn parse_multiplicative(&mut self) -> Result<AstNode, Error> {
        let mut left = self.parse_power()?;

        while let Some(Token::Operator(op)) = self.current() {
            if *op != '*' && *op != '/' {
                break;
            }
            let op = *op;
            self.advance();
            let right = self.parse_power()?;
            left = AstNode::BinaryOp {
                op,
                left: Box::new(left),
                right: Box::new(right),
            };
        }

        Ok(left)
    }

    /// 解析幂运算（优先级 3，右结合）
    fn parse_power(&mut self) -> Result<AstNode, Error> {
        let left = self.parse_unary()?;

        if let Some(Token::Operator('^')) = self.current() {
            self.advance();
            let right = self.parse_power()?; // 右结合
            return Ok(AstNode::BinaryOp {
                op: '^',
                left: Box::new(left),
                right: Box::new(right),
            });
        }

        Ok(left)
    }

    /// 解析一元运算符（优先级 4）
    fn parse_unary(&mut self) -> Result<AstNode, Error> {
        // 处理 +/- 一元运算符
        if let Some(Token::Operator(op)) = self.current() {
            if *op == '+' || *op == '-' {
                let op = *op;
                self.advance();
                let operand = self.parse_unary()?;
                return Ok(AstNode::UnaryOp {
                    op,
                    operand: Box::new(operand),
                });
            }
        }

        // 处理前缀运算符（如 √）
        if let Some(Token::PrefixOp(name)) = self.current() {
            let name = name.clone();
            self.advance();
            let operand = self.parse_unary()?;
            return Ok(AstNode::FunctionCall {
                name,
                args: vec![operand],
            });
        }

        self.parse_primary()
    }

    /// 解析基本元素（数字、括号、函数调用）
    fn parse_primary(&mut self) -> Result<AstNode, Error> {
        match self.current() {
            Some(Token::Number(n)) => {
                let n = *n;
                self.advance();
                Ok(AstNode::Number(n))
            }

            Some(Token::LeftParen) => {
                self.advance(); // 跳过 '('
                let expr = self.parse_expression()?;

                match self.current() {
                    Some(Token::RightParen) => {
                        self.advance(); // 跳过 ')'
                        Ok(expr)
                    }
                    _ => Err(Error::ParseError("缺少右括号".to_string())),
                }
            }

            Some(Token::Function(name)) => {
                let name = name.clone();
                self.advance();

                // 期望左括号
                match self.current() {
                    Some(Token::LeftParen) => {
                        self.advance();
                    }
                    _ => {
                        return Err(Error::ParseError(format!("函数 {} 后缺少左括号", name)));
                    }
                }

                // 解析参数列表
                let mut args = Vec::new();

                // 检查是否为空参数列表
                if let Some(Token::RightParen) = self.current() {
                    self.advance();
                    return Ok(AstNode::FunctionCall { name, args });
                }

                // 解析第一个参数
                args.push(self.parse_expression()?);

                // 解析剩余参数（逗号分隔）
                // 注意：这个简化版本暂不支持逗号，只支持单参数函数

                // 期望右括号
                match self.current() {
                    Some(Token::RightParen) => {
                        self.advance();
                        Ok(AstNode::FunctionCall { name, args })
                    }
                    _ => Err(Error::ParseError(format!(
                        "函数 {} 参数列表缺少右括号",
                        name
                    ))),
                }
            }

            Some(Token::Constant(name)) => {
                let name = name.clone();
                self.advance();

                // 返回常量对应的数值
                let value = match name.as_str() {
                    "pi" => std::f64::consts::PI,
                    "e" => std::f64::consts::E,
                    _ => return Err(Error::ParseError(format!("未知常量: {}", name))),
                };

                Ok(AstNode::Number(value))
            }

            _ => Err(Error::ParseError(format!(
                "意外的 token: {:?}",
                self.current()
            ))),
        }
    }
}

impl Default for ExpressionParser {
    fn default() -> Self {
        Self::new()
    }
}

/// 抽象语法树节点
#[derive(Debug, Clone, PartialEq)]
pub enum AstNode {
    /// 数字节点
    Number(f64),
    /// 二元操作节点
    BinaryOp {
        op: char,
        left: Box<AstNode>,
        right: Box<AstNode>,
    },
    /// 一元操作节点
    UnaryOp { op: char, operand: Box<AstNode> },
    /// 函数调用节点
    FunctionCall { name: String, args: Vec<AstNode> },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_addition() {
        let parser = ExpressionParser::new();
        let tokens = parser.tokenize("1 + 2").unwrap();
        let ast = parser.parse(tokens).unwrap();

        match ast {
            AstNode::BinaryOp { op, .. } => assert_eq!(op, '+'),
            _ => panic!("Expected BinaryOp"),
        }
    }

    #[test]
    fn test_parse_operator_precedence() {
        let parser = ExpressionParser::new();
        // 2 + 3 * 4 应该解析为 2 + (3 * 4)
        let tokens = parser.tokenize("2 + 3 * 4").unwrap();
        let ast = parser.parse(tokens).unwrap();

        match ast {
            AstNode::BinaryOp { op, left, right } => {
                assert_eq!(op, '+');
                assert!(matches!(*left, AstNode::Number(2.0)));
                assert!(matches!(*right, AstNode::BinaryOp { op: '*', .. }));
            }
            _ => panic!("Expected BinaryOp"),
        }
    }

    #[test]
    fn test_parse_parentheses() {
        let parser = ExpressionParser::new();
        // (2 + 3) * 4
        let tokens = parser.tokenize("(2 + 3) * 4").unwrap();
        let ast = parser.parse(tokens).unwrap();

        match ast {
            AstNode::BinaryOp { op, left, .. } => {
                assert_eq!(op, '*');
                assert!(matches!(*left, AstNode::BinaryOp { op: '+', .. }));
            }
            _ => panic!("Expected BinaryOp"),
        }
    }

    #[test]
    fn test_parse_function() {
        let parser = ExpressionParser::new();
        let tokens = parser.tokenize("sin(3.14)").unwrap();
        let ast = parser.parse(tokens).unwrap();

        match ast {
            AstNode::FunctionCall { name, args } => {
                assert_eq!(name, "sin");
                assert_eq!(args.len(), 1);
            }
            _ => panic!("Expected FunctionCall"),
        }
    }

    #[test]
    fn test_parse_unary_minus() {
        let parser = ExpressionParser::new();
        let tokens = parser.tokenize("-5").unwrap();
        let ast = parser.parse(tokens).unwrap();

        match ast {
            AstNode::UnaryOp { op, .. } => assert_eq!(op, '-'),
            _ => panic!("Expected UnaryOp"),
        }
    }

    #[test]
    fn test_tokenize_simple() {
        let parser = ExpressionParser::new();
        let tokens = parser.tokenize("1 + 2").unwrap();

        assert_eq!(tokens.len(), 3);
        assert_eq!(tokens[0], Token::Number(1.0));
        assert_eq!(tokens[1], Token::Operator('+'));
        assert_eq!(tokens[2], Token::Number(2.0));
    }

    #[test]
    fn test_tokenize_with_parentheses() {
        let parser = ExpressionParser::new();
        let tokens = parser.tokenize("(3 + 4) * 5").unwrap();

        assert_eq!(tokens.len(), 7);
        assert_eq!(tokens[0], Token::LeftParen);
        assert_eq!(tokens[1], Token::Number(3.0));
        assert_eq!(tokens[6], Token::Number(5.0));
    }

    #[test]
    fn test_tokenize_decimal() {
        let parser = ExpressionParser::new();
        let tokens = parser.tokenize("3.14 + 2.5").unwrap();

        assert_eq!(tokens[0], Token::Number(3.14));
        assert_eq!(tokens[2], Token::Number(2.5));
    }

    #[test]
    fn test_tokenize_function() {
        let parser = ExpressionParser::new();
        let tokens = parser.tokenize("sin(3.14)").unwrap();

        assert_eq!(tokens[0], Token::Function("sin".to_string()));
        assert_eq!(tokens[1], Token::LeftParen);
        assert_eq!(tokens[2], Token::Number(3.14));
    }

    #[test]
    fn test_tokenize_unicode_sqrt() {
        let parser = ExpressionParser::new();
        let tokens = parser.tokenize("√16").unwrap();

        assert_eq!(tokens[0], Token::PrefixOp("sqrt".to_string()));
        assert_eq!(tokens[1], Token::Number(16.0));
    }

    #[test]
    fn test_tokenize_unicode_operators() {
        let parser = ExpressionParser::new();
        let tokens = parser.tokenize("5 × 3 ÷ 2").unwrap();

        assert_eq!(tokens[1], Token::Operator('*'));
        assert_eq!(tokens[3], Token::Operator('/'));
    }

    #[test]
    fn test_tokenize_pi() {
        let parser = ExpressionParser::new();
        let tokens = parser.tokenize("π").unwrap();

        assert_eq!(tokens[0], Token::Constant("pi".to_string()));
    }

    #[test]
    fn test_tokenize_superscript() {
        let parser = ExpressionParser::new();
        let tokens = parser.tokenize("5²").unwrap();

        assert_eq!(tokens[0], Token::Number(5.0));
        assert_eq!(tokens[1], Token::Operator('^'));
        assert_eq!(tokens[2], Token::Number(2.0));
    }

    #[test]
    fn test_parse_unicode_sqrt() {
        let parser = ExpressionParser::new();
        let tokens = parser.tokenize("√16").unwrap();
        let ast = parser.parse(tokens).unwrap();

        match ast {
            AstNode::FunctionCall { name, .. } => assert_eq!(name, "sqrt"),
            _ => panic!("Expected FunctionCall"),
        }
    }

    #[test]
    fn test_parse_pi() {
        let parser = ExpressionParser::new();
        let tokens = parser.tokenize("π + 1").unwrap();
        let ast = parser.parse(tokens).unwrap();

        match ast {
            AstNode::BinaryOp { op, .. } => assert_eq!(op, '+'),
            _ => panic!("Expected BinaryOp"),
        }
    }

    #[test]
    fn test_invalid_character() {
        let parser = ExpressionParser::new();
        let result = parser.tokenize("1 @ 2");

        assert!(result.is_err());
    }
}
