// src/lib.rs
pub mod utils;

//导出引入`utils`模块的`greet`函数，以便在其他地方使用
pub use utils::greet;

// 模块`math`的声明，包含数学相关功能
pub mod math {
    // 子模块`operations`的声明，包含数学运算相关功能
    pub mod operations;
}
