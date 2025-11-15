fn ownership_demo() {
    // 所有权转移
    let data = vec![1, 2, 3];
    let transferred = data; // data 移动到 transferred

    // println!("{:?}", data); // 编译错误！data 已经移动
    println!("{:?}", transferred); // 正常

    // 借用（引用）
    let reference = &transferred;
    println!("引用值: {:?}", reference); // 读取操作不需要所有权

    // 多重引用
    let another_ref = &transferred;
    // 但是不能有可变引用
    // let mut_ref = &mut transferred; // 编译错误！

    println!("两个引用: {:?}, {:?}", reference, another_ref);
}
fn main() {
    println!("=== ownership demo ===");
    ownership_demo();
    println!("=== ownership demo end! ===");
}
