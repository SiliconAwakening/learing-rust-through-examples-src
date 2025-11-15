fn integer_types() {
    // 有符号整数
    let i8_val: i8 = -128; // 范围: -128 到 127
    let i16_val: i16 = -32768; // 范围: -32768 到 32767
    let i32_val: i32 = -2147483648; // 默认的整数类型
    let i64_val: i64 = -9223372036854775808;
    let i128_val: i128 = -170141183460469231731687303715884105728;

    // 无符号整数
    let u8_val: u8 = 255; // 范围: 0 到 255
    let u16_val: u16 = 65535;
    let u32_val: u32 = 4294967295;
    let u64_val: u64 = 18446744073709551615;
    let u128_val: u128 = 340282366920938463463374607431768211455;

    // 平台相关整数类型
    let isize: isize = -1; // 平台相关大小
    let usize: usize = 1; // 平台相关大小

    // 数值字面量
    let decimal = 98_222; // 十进制（可用下划线分隔）
    let hex = 0xff; // 十六进制
    let octal = 0o77; // 八进制
    let binary = 0b1111_0000; // 二进制
    let byte = b'A'; // 字节字符（仅 u8）

    println!("整数值: {}, {}, {}, {}", decimal, hex, octal, binary);
}

fn main() {
    println!("=== integer types ===");
    integer_types();
    println!("=== integer types end! ===");
}
