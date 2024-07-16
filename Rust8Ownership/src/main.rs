/*
Rust 所有权
计算机程序必须在运行时管理它们所使用的内存资源。

大多数的编程语言都有管理内存的功能：

C/C++ 这样的语言主要通过手动方式管理内存，开发者需要手动的申请和释放内存资源。
但为了提高开发效率，只要不影响程序功能的实现，
许多开发者没有及时释放内存的习惯。所以手动管理内存的方式常常造成资源浪费。

Java 语言编写的程序在虚拟机（JVM）中运行，JVM 具备自动回收内存资源的功能。
但这种方式常常会降低运行时效率，所以 JVM 会尽可能少的回收资源，这样也会使程序占用较大的内存资源。

所有权对大多数开发者而言是一个新颖的概念，它是 Rust 语言为高效使用内存而设计的语法机制。
所有权概念是为了让 Rust 在编译阶段更有效地分析内存资源的有用性以实现内存管理而诞生的概念。

所有权规则
所有权有以下三条规则：

Rust 中的每个值都有一个变量，称为其所有者。
一次只能有一个所有者。
当所有者不在程序运行范围时，该值将被删除。
这三条规则是所有权概念的基础。

接下来将介绍与所有权概念有关的概念。

变量范围
我们用下面这段程序描述变量范围的概念：

{
    // 在声明以前，变量 s 无效
    let s = "runoob";
    // 这里是变量 s 的可用范围
}
// 变量范围已经结束，变量 s 无效
变量范围是变量的一个属性，其代表变量的可行域，默认从声明变量开始有效直到变量所在域结束。
*/
//参考文档 https://www.runoob.com/rust/rust-ownership.html

//函数所有权相关
mod function;
//引用与租借 的所有权相关
mod refrence_lease;
fn main() {
    println!("Hello, world!");
    // clone_();
    // function::print_function_ownership();
    // function::retur_ownership();
    // refrence_lease::example1();
    // refrence_lease::example2();
    // refrence_lease::example3();
    refrence_lease::not_variable_and_variable();
}

// 所有权规则 克隆
fn clone_() {
    //这里是真的将堆中的 "hello" 复制了一份，所以 s1 和 s2 都分别绑定了一个值，释放的时候也会被当作两个资源。
    //当然，克隆仅在需要复制的情况下使用，毕竟复制数据会花费更多的时间。
    let s1 = String::from("hello");
    let s2 = s1.clone(); //相当于重新创建了一个值在内存空间, 相当于 s1 的副本
    println!("s1 = {}, s2 = {}", s1, s2);
}
// 所有权规则 移动
fn move_() {
    {
        // x 和 y 作用范围在{} 出了范围，x 和 y 变量就无效了
        let x = 5;
        let y = x;
    }
    //println!("x :{}, y: {}", x,y);  // 错误！x , y  已经失效
    let s1 = String::from("hello");
    let s2 = s1;
    // println!("{}, world!", s1); // 错误！s1 已经失效
    println!("{}, world!", s2);
}
