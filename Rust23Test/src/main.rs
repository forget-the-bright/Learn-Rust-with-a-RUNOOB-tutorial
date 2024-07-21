fn main() {
    println!("Hello, world!");
}

/*
在 Rust 中，测试代码通常与生产代码放在同一个文件中，但位于特定的 #[cfg(test)] 模块中。这样做可以确保测试代码只在执行测试时编译和运行。
*/
#[cfg(test)]
mod tests;
