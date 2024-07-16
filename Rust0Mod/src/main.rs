// src/main.rs

// 引入库 crate 的根模块
use Rust0Mod;
// 定义本地根的 utils 模块
mod utils;

// 主函数，程序的入口点
fn main() {
    // 使用别名导入模块中的函数，以简化调用
    use Rust0Mod::math::operations::add;
    // 调用 crate 根模块下的函数，向其传递参数 "123"
    crate::Rust0Mod::greet("123");
    // 调用 utils 模块下的函数，向其传递参数 "123"
    crate::Rust0Mod::utils::greet("123");
    // 调用本地 utils 模块下的函数，向其传递参数 "123"
    utils::greet("123");

    // 调用导入的 add 函数并传入参数 10 和 20，将返回值存储在 result 变量中
    let result = add(10, 20);
    // 打印调用 add 函数的结果
    println!("Result from add: {}", result);

    // 使用别名重新导入同一个函数，以演示命名冲突的解决
    use Rust0Mod::math::operations::add as custom_add;
    // 调用重新命名的 add 函数并传入参数 30 和 40，将返回值存储在 custom_result 变量中
    let custom_result = custom_add(30, 40);
    // 打印调用 custom_add 函数的结果
    println!("Result from custom_add: {}", custom_result);

    // 调用 lib_add_example 函数，该函数可能在演示或测试其他功能
    Rust0Mod::math::operations::lib_add_example();
}
