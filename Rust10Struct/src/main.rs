/*
Rust 结构体
Rust 中的结构体（Struct）与元组（Tuple）都可以将若干个类型不一定相同的数据捆绑在一起形成整体，
但结构体的每个成员和其本身都有一个名字，这样访问它成员的时候就不用记住下标了。
元组常用于非定义的多值传递，而结构体用于规范常用的数据结构。结构体的每个成员叫做"字段"。

结构体定义
这是一个结构体定义：

struct Site {
    domain: String,
    name: String,
    nation: String,
    found: u32
}

注意：如果你常用 C/C++，请记住在 Rust 里 struct 语句仅用来定义，
不能声明实例，结尾不需要 ; 符号，而且每个字段定义之后用 , 分隔。
*/
mod defineStruct;
fn main() {
    // defineStruct::print_struct();
    // defineStruct::print_struct2();
    // defineStruct::tupleStruct::print_tuple_struct();
    defineStruct::structMethod::print_struct_method(30, 50);
}
