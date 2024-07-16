fn main() {
    println!("Hello, world!");
    example1();
    example2(3);
    imitate_ternary();
}

//Rust 语言中没有传统意义上的三元操作符，即没有 a ? b : c 这样的形式
//在 Rust 中我们可以使用 if-else 结构实现类似于三元条件运算表达式 (A ? B : C) 的效果
fn imitate_ternary() {
    let a = 3;
    let number = (if a > 0 { 1 } else { -1 }) * a;
    println!("number 为 {}", number);
}
/*
Rust 中的条件表达式必须是 bool 类型
虽然 C/C++ 语言中的条件表达式用整数表示，非 0 即真，但这个规则在很多注重代码安全性的语言中是被禁止的。
if <condition> { block 1 } else { block 2 }

*/

// else-if 语法
fn example2(a: i32) {
    let mut print_str_ = "";
    if a == 1 {
        print_str_ = "a 等于 1";
    } else if a == 2 {
        print_str_ = "a 等于 2";
    } else {
        print_str_ = "a 不等于 1 或 2";
    }
    println!("{}", print_str_);
}
//条件判断基本语法
fn example1() {
    let number = 3;
    if number < 5 {
        println!("条件为 true");
    } else {
        println!("条件为 false");
    }
}
