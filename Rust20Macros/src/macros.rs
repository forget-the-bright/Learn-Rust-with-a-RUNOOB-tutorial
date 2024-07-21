/*
宏的定义
在 Rust 中，使用 macro_rules! 关键字来定义声明式宏。

macro_rules! my_macro {
    // 模式匹配和展开
    ($arg:expr) => {
        // 生成的代码
        // 使用 $arg 来代替匹配到的表达式
    };
}
声明式宏使用 macro_rules! 关键字进行定义，它们被称为 "macro_rules" 宏。
这种宏的定义是基于模式匹配的，可以匹配代码的结构并根据匹配的模式生成相应的代码。
这样的宏在不引入新的语法结构的情况下，可以用来简化一些通用的代码模式。

*/
// 宏的定义  #[macro_export] 宏的导出相当于 pub 开发访问
#[macro_export]
macro_rules! greet {
    // 模式匹配
    ($name:expr) => {
        // 宏的展开
        println!("Hello, {}!", $name);
    };
}
/*
说明

模式匹配：
宏通过模式匹配来匹配传递给宏的代码片段，模式是宏规则的左侧部分，用于捕获不同的代码结构。
规则：宏规则是一组由 $ 引导的模式和相应的展开代码，规则由分号分隔。
宏的展开：当宏被调用时，匹配的模式将被替换为相应的展开代码，展开代码是宏规则的右侧部分。
*/
// 宏的定义
#[macro_export]
macro_rules! my_vec {
    // 基本情况，空的情况
    () => {
        Vec::new()
    };

    // 递归情况，带有元素的情况
    ($($element:expr),+ $(,)?) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($element);
            )+
            temp_vec
        }
    };
    /*
    在这个例子中，vec! 宏使用了模式匹配，以及 $($element:expr),+ $(,)?)

    这样的语法来捕获传递给宏的元素，并用它们创建一个 Vec。

    注意，$(,)?) 用于处理末尾的逗号，使得在不同的使用情境下都能正常工作。
    */
}
pub fn use_my_vec_macro() {
    let v = my_vec![1, 2, 3];
    println!("{:?}", v);
}
/*
过程宏（Procedural Macros）

过程宏是一种更为灵活和强大的宏，允许在编译时通过自定义代码生成过程来操作抽象语法树（AST）。
过程宏在功能上更接近于函数，但是它们在编写和使用上更加复杂。

过程宏的类型：
派生宏（Derive Macros）：用于自动实现trait（比如Copy、Debug）的宏。
属性宏（Attribute Macros）：用于在声明上附加额外的元数据，如  #[derive(Debug)]。

过程宏的实现通常需要使用 proc_macro 库提供的功能，例如 TokenStream 和 TokenTree，以便更直接地操纵源代码。
*/
