fn main() {
    println!("Hello, world!");
    Shadowing();
    notMutable_and_mutable();
    constant_and_mutable()
}
//不可变变量与可变变量的区别

fn notMutable_and_mutable() {
    let x = 5;
    // x = 6; 不合法
    let mut y = 5;
    y = 6;
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}
//常量与不可变变量的区别
fn constant_and_mutable() {
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    // let THREE_HOURS_IN_SECONDS = 456; 不合法
    let x = 5;
    let x = x + 1;
    println!("The value of x is: {}", x);
}

//重影的例子
/*
重影的概念与其他面向对象语言里的"重写"（Override）或"重载"（Overload）是不一样的。
重影就是刚才讲述的所谓"重新绑定"，之所以加引号就是为了在没有介绍这个概念的时候代替一下概念。
重影就是指变量的名称可以被重新使用的机制
*/
fn Shadowing() {
    let x = 5;
    let x = x + 1;
    let mut x = x * 2;
    x += 2;
    println!("The value of x is: {}", x);

    let mut s = "123";
    // s = s.len();  // 不合法，赋值时类型不匹配
}
