// 计算器模块
// 演示如何在 examples 中创建多文件项目

pub struct Calculator {
    name: String,
}

impl Calculator {
    /// 创建新的计算器实例
    pub fn new() -> Self {
        Calculator {
            name: String::from("Basic Calculator"),
        }
    }

    /// 加法运算
    pub fn add(&self, a: i32, b: i32) -> i32 {
        a + b
    }

    /// 减法运算
    pub fn subtract(&self, a: i32, b: i32) -> i32 {
        a - b
    }

    /// 乘法运算
    pub fn multiply(&self, a: i32, b: i32) -> i32 {
        a * b
    }

    /// 除法运算
    pub fn divide(&self, a: i32, b: i32) -> Option<i32> {
        if b == 0 {
            None
        } else {
            Some(a / b)
        }
    }

    /// 获取计算器名称
    pub fn get_name(&self) -> &str {
        &self.name
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
    fn test_add() {
        let calc = Calculator::new();
        assert_eq!(calc.add(2, 3), 5);
    }

    #[test]
    fn test_divide() {
        let calc = Calculator::new();
        assert_eq!(calc.divide(10, 2), Some(5));
        assert_eq!(calc.divide(10, 0), None);
    }
}
