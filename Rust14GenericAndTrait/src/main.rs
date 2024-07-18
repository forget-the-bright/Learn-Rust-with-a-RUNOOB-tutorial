/*
Rust 泛型与特性
泛型是一个编程语言不可或缺的机制。

C++ 语言中用"模板"来实现泛型，而 C 语言中没有泛型的机制，这也导致 C 语言难以构建类型复杂的工程。

泛型机制是编程语言用于表达类型抽象的机制，一般用于功能确定、数据类型待定的类，如链表、映射表等。
*/
mod function_Generics;
mod struct_enum_Generics;
mod trait_info;
fn main() {
    // function_Generics::function_Generics();
    // struct_enum_Generics::struct_example();
    // struct_enum_Generics::enum_example();
    // trait_info::default_trait();
    trait_info::multi_trait();
}
