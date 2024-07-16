/*
Rust 枚举类
枚举类在 Rust 中并不像其他编程语言中的概念那样简单，但依然可以十分简单的使用：
*/
#[derive(Debug)]
enum Book1 {
    //基础用法
    Papery,
    Electronic,
}
#[derive(Debug)]
pub enum Book2 {
    //可以为枚举类成员添加元组属性描述
    Papery(u32),
    Electronic(String),
}

#[derive(Debug)]
enum Book3 {
    //为属性命名，可以用结构体语法来
    //虽然可以如此命名，但请注意，并不能像访问结构体字段一样访问枚举类绑定的属性。访问的方法在 match 语法中。
    Papery { index: u32 },
    Electronic { url: String },
}
mod Option;
fn main() {
    // print_enum1();
    // print_enum2();
    // print_enum3();
    // print_match();
    // print_match2();
    // print_match3();
    // Option::example1();
    // Option::example2();
    // Option::example3();
    Option::example4();
}
/*
match 语法
枚举的目的是对某一类事物的分类，分类的目的是为了对不同的情况进行描述。
基于这个原理，往往枚举类最终都会被分支结构处理（许多语言中的 switch ）。
switch 语法很经典，但在 Rust 中并不支持，
很多语言摒弃 switch 的原因都是因为 switch 容易存在因忘记添加 break 而产生的串接运行问题，
Java 和 C# 这类语言通过安全检查杜绝这种情况出现。
Rust 通过 match 语句来实现分支结构。先认识一下如何用 match 处理枚举类


match 块也可以当作函数表达式来对待，它也是可以有返回值的但是所有返回值表达式的类型必须一样！
match 枚举类实例 {
    分类1 => 返回值表达式,
    分类2 => 返回值表达式,
    ...
}
*/
fn print_match3() {
    /*
    match 除了能够对枚举类进行分支选择以外，还可以对整数、浮点数、字符和字符串切片引用（&str）类型的数据进行分支选择。
    其中，浮点数类型被分支选择虽然合法，但不推荐这样使用，因为精度问题可能会导致分支错误。
    对非枚举类进行分支选择时必须注意处理例外情况，即使在例外情况下没有任何要做的事 . 例外情况用下划线 _ 表示
     */
    let t = "abc";
    match t {
        "abc" => println!("Yes"),
        _ => {}
    }
}
fn print_match2() {
    let book = Book2::Papery(1001);
    let ebook = Book2::Electronic(String::from("url://..."));
    match book {
        Book2::Papery(i) => {
            println!("{}", i);
        }
        Book2::Electronic(url) => {
            println!("{}", url);
        }
    }
}
fn print_match1() {
    let book = Book3::Papery { index: 1001 };
    let ebook = Book3::Electronic {
        url: String::from("url://..."),
    };
    /* match book {
        Book3::Papery { index } => {
            println!("Papery book {}", index);
        }
        Book3::Electronic { url } => {
            println!("E-book {}", url);
        }
    } */
    let val = match book {
        Book3::Papery { index } => format!("Papery book {}", index),
        Book3::Electronic { url } => format!("E-book {}", url),
    };
    println!("{}", val)
}
fn print_enum3() {
    let book = Book3::Papery { index: 1001 };
    let ebook = Book3::Electronic {
        url: String::from("url://..."),
    };
    println!("{:?} , {:?}", book, ebook);
}
fn print_enum2() {
    let book = Book2::Papery(1001);
    let ebook = Book2::Electronic(String::from("url://..."));
    println!("{:?} , {:?}", book, ebook);
}
fn print_enum1() {
    let book = Book1::Papery;
    println!("{:?}", book);
}
