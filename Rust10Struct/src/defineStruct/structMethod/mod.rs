/*
结构体方法
方法（Method）和函数（Function）类似，只不过它是用来操作结构体实例的。

如果你学习过一些面向对象的语言，那你一定很清楚函数一般放在类定义里并在函数中用 this 表示所操作的实例。

Rust 语言不是面向对象的，从它所有权机制的创新可以看出这一点。但是面向对象的珍贵思想可以在 Rust 实现。

结构体方法的第一个参数必须是 &self，不需声明类型，因为 self 不是一种风格而是关键字。
*/
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

pub fn print_struct_method(width: u32, height: u32) {
    let rect1 = Rectangle { width, height };
    println!("Hello, Struct rect1! {:#?}", rect1);
    println!("rect1's area is {}", rect1.area());
}
//attention 注意：结构体 impl 块可以写几次，效果相当于它们内容的拼接！
