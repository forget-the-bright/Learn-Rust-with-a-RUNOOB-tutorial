// src/math/operations.rs

// 定义一个公共函数，可以在其他模块中使用
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

// 在库 crate 的根模块中使用绝对路径引用 operations 模块
pub fn lib_add_example() {
    let result = crate::math::operations::add(5, 3);
    println!("Result from lib_add_example: {}", result);
}
