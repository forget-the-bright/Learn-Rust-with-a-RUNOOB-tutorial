/*
元组结构体

有一种更简单的定义和使用结构体的方式：元组结构体。

元组结构体是一种形式是元组的结构体。

与元组的区别是它有名字和固定的类型格式。它存在的意义是为了处理那些需要定义类型（经常使用）又不想太复杂的简单数据
*/
#[derive(Debug)]
struct Color(u8, u8, u8);
#[derive(Debug)]
struct Point(f64, f64);

pub fn print_tuple_struct() {
    let black = Color(0, 0, 0);
    let origin = Point(0.0, 0.0);

    println!("Hello, TupleStruct black! {:#?}", black);
    println!("Hello, TupleStruct origin! {:#?}", origin);
}
