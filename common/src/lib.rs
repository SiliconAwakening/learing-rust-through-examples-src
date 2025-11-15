// Common library for the Rust from Zero project
// 本库提供所有章节共享的工具函数和类型定义

/// 打印章节标题
pub fn print_chapter_header(chapter: &str, title: &str) {
    println!("\n{}", "=".repeat(60));
    println!("  {} - {}", chapter, title);
    println!("{}\n", "=".repeat(60));
}

/// 打印章节结束信息
pub fn print_chapter_footer() {
    println!("\n{}", "=".repeat(60));
    println!("  示例运行完成！");
    println!("{}\n", "=".repeat(60));
}

/// 打印分节标题
pub fn print_section(number: u32, title: &str) {
    println!("\n{}. {}:", number, title);
    println!("{}", "-".repeat(40));
}

/// 打印带缩进的信息
pub fn print_info(message: &str) {
    println!("   {}", message);
}

/// 格式化输出代码
pub fn print_code(code: &str) {
    println!("   >>> {}", code);
}

/// 格式化输出结果
pub fn print_result(result: &str) {
    println!("   => {}", result);
}

/// 计算两个数的最大公约数（欧几里得算法）
pub fn gcd(mut a: u32, mut b: u32) -> u32 {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}

/// 判断是否为质数
pub fn is_prime(n: u32) -> bool {
    if n < 2 {
        return false;
    }
    if n == 2 {
        return true;
    }
    if n.is_multiple_of(2) {
        return false;
    }

    let sqrt_n = (n as f64).sqrt() as u32;
    for i in (3..=sqrt_n).step_by(2) {
        if n.is_multiple_of(i) {
            return false;
        }
    }
    true
}

/// 计算斐波那契数列第 n 项
pub fn fibonacci(n: u32) -> u64 {
    match n {
        0 => 0,
        1 => 1,
        _ => {
            let mut a = 0u64;
            let mut b = 1u64;
            for _ in 2..=n {
                let temp = a + b;
                a = b;
                b = temp;
            }
            b
        }
    }
}

/// 将字符串转换为标题格式（首字母大写）
pub fn to_title_case(s: &str) -> String {
    s.split_whitespace()
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                None => String::new(),
                Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gcd() {
        assert_eq!(gcd(48, 18), 6);
        assert_eq!(gcd(100, 50), 50);
        assert_eq!(gcd(17, 13), 1);
    }

    #[test]
    fn test_is_prime() {
        assert!(!is_prime(0));
        assert!(!is_prime(1));
        assert!(is_prime(2));
        assert!(is_prime(3));
        assert!(!is_prime(4));
        assert!(is_prime(17));
        assert!(!is_prime(100));
    }

    #[test]
    fn test_fibonacci() {
        assert_eq!(fibonacci(0), 0);
        assert_eq!(fibonacci(1), 1);
        assert_eq!(fibonacci(5), 5);
        assert_eq!(fibonacci(10), 55);
    }

    #[test]
    fn test_to_title_case() {
        assert_eq!(to_title_case("hello world"), "Hello World");
        assert_eq!(to_title_case("rust programming"), "Rust Programming");
    }
}
