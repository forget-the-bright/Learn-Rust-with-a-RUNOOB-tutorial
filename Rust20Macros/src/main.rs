/*
Rust 宏
Rust 宏（Macros）是一种在编译时生成代码的强大工具，它允许你在编写代码时创建自定义语法扩展。

宏（Macro）是一种在代码中进行元编程（Metaprogramming）的技术，它允许在编译时生成代码，
宏可以帮助简化代码，提高代码的可读性和可维护性，同时允许开发者在编译时执行一些代码生成的操作。

宏在 Rust 中有两种类型：声明式宏（Declarative Macros）和过程宏（Procedural Macros）。
本文主要介绍声明式宏。
*/

mod macros;
fn main() {
    greet!("Hello, world!");
    macros::use_my_vec_macro();
}
