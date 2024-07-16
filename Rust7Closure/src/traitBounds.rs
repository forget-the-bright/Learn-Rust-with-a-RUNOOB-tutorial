/*
在 Rust 编程语言中，Fn、FnMut 和 FnOnce 是 trait 边界（trait bounds），
它们用于指定一个类型必须实现哪个 trait。这些 trait 边界定义了闭包（closure）可以被调用的次数，以及是否可以修改其捕获的变量。

以下是这三个 trait 边界的主要区别：

FnOnce:
    FnOnce 闭包只能被调用一次。
    闭包可以捕获其作用域内的不可变变量。
    闭包在第一次调用后不能再次调用。

FnMut:
    FnMut 闭包可以被多次调用。
    闭包可以捕获其作用域内的可变变量。
    闭包在每次调用时可以修改其捕获的可变变量。

Fn:
    Fn 闭包可以被多次调用。
    闭包可以捕获其作用域内的不可变变量。
    闭包在每次调用时不能修改其捕获的变量。
    这些 trait 边界通常用于函数签名，以指定闭包的行为。例如，如果你定义一个函数，它接受一个闭包作为参数，你可能需要指定这个闭包必须实现哪个 trait。

以下是一个使用 FnOnce、FnMut 和 Fn trait 边界的示例：
*/
pub fn apply<F>(f: F, x: i32)
where
    F: Fn(i32) -> i32,
{
    let result = f(x);
    println!("The result is: {}", result);
}

/*
在 Rust 编程语言中，Box 是一个特殊的类型，它代表一个堆（heap）分配的值。Box 通常用于动态分配的内存，例如在堆上分配的动态数组。

以下是一些关于 Box 的关键点：

动态分配：Box 用于在堆上分配内存，而不是在栈上。这允许你创建一个可以在程序的任何部分访问的大数据结构。
类型参数：Box 有一个类型参数，它表示被包装的值的类型。例如，Box<i32> 表示一个在堆上分配的 i32 类型的值。
引用计数：Box 通常与 Rc（引用计数类型）一起使用，以确保在多个地方引用同一个值时不会出现内存泄漏。
移动和所有权：当一个值被包装在 Box 中时，它将失去所有权，这意味着原始值不再有效，因为它的所有权被转移到了 Box。
解包：如果你想访问 Box 包装的值，你需要将其解包。这可以通过调用 Box 类型的 .into_inner() 方法或直接解包（使用 Box::new(x) 语法）来实现。
*/
pub fn applyReturn<F>(f: F, x: i32) -> Box<dyn Fn(i32) -> i32>
where
    F: Fn(i32) -> i32,
{
    let result = f(x);
    return Box::new(move |y: i32| {
        println!("The result is: {}", result);
        return result + y;
    });
}

pub fn applyOnce<F>(f: F)
where
    F: FnOnce(i32) -> i32,
{
    f(42);
}

//参数f 必须是用mut 修饰的闭包
pub fn apply_mut<F>(f: &mut F)
where
    F: FnMut(i32) -> i32,
{
    f(42);
}
//参数f 必须是用mut 修饰的闭包
pub fn apply_mut_result<F>(f: &mut F) -> i32
where
    F: FnMut(i32) -> i32,
{
    f(42)
}
