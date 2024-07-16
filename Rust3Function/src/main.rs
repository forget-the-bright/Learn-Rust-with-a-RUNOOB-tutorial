use std::any::type_name_of_val;
fn main() {
    println!("Hello, world!");
    // another_function();
    // another_function2(2, 3);
    //function_body_expression();
    println!("{}", return_function());
    nested_function();
}

//嵌套函数
fn nested_function() {
    fn five() -> i32 {
        5
    }
    println!(
        "five() 的值为: {} | five type {}",
        five(),
        type_name_of_val(&five)
    );
}
//有返回值函数
fn return_function() -> i32 {
    5 //可以省略return 关键字
}
//空返回值函数 ->() 可以省略
fn empty_return_function() -> () {}

//函数体表达式
fn function_body_expression() {
    let x = 5;
    //函数体表达式
    let y = {
        let b = 3;
        b + 1
    };
    //println!("b 的值为 : {}", b);  b值作用域在 y {} 里面
    println!("x 的值为 : {}", x);
    println!("y 的值为 : {}", y);
}
/*
Rust 函数的基本形式：
fn <函数名> ( <参数> ) <函数体>
其中 Rust 函数名称的命名风格是小写字母以下划线分割：
*/
fn another_function() {
    println!("Hello, runoob!");
}

fn another_function2(x: i32, y: i32) {
    println!("x 的值为 : {}", x);
    println!("y 的值为 : {}", y);
}
