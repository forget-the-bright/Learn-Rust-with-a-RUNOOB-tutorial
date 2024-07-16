/*
浮点数型（Floating-Point）
Rust 与其它语言一样支持  32 位浮点数（f32）和 64 位浮点数（f64）。
默认情况下， 64.0 将表示 64 位浮点数，即浮点数默认都为 f64。
因为现代计算机处理器对两种浮点数计算的速度几乎相同，但 64 位浮点数精度更高。
*/
use std::any::type_name_of_val;
pub fn printFloatingPoint() {
    let x = 2.0; // f64
    let y: f32 = 3.0; // f32
    println!(
        "let x = 2.0: {}, let y: f32 = 3.0: {}",
        type_name_of_val(&x),
        type_name_of_val(&y)
    );
}
