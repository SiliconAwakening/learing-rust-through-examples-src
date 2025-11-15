// 工具函数模块

/// 打印分隔线
pub fn print_separator() {
    println!("\n{}\n", "-".repeat(40));
}

/// 格式化消息
pub fn format_message(msg: &str) -> String {
    format!(">>> {}", msg)
}

/// 打印标题
pub fn print_title(title: &str) {
    println!("\n=== {} ===", title);
}

/// 重复字符串
pub fn repeat_str(s: &str, times: usize) -> String {
    s.repeat(times)
}
