/*
Rust 闭包
Rust 中的闭包是一种匿名函数，它们可以捕获并存储其环境中的变量。

闭包允许在其定义的作用域之外访问变量，并且可以在需要时将其移动或借用给闭包。

闭包在 Rust 中被广泛应用于函数式编程、并发编程和事件驱动编程等领域。

闭包在 Rust 中非常有用，因为它们提供了一种简洁的方式来编写和使用函数。

以下是 Rust 闭包的一些关键特性和用法：

闭包的语法声明：

|参数...| { 表达式 }

参数可以有类型注解，也可以省略，Rust 编译器会根据上下文推断它们。

let add_one = |x: i32| x + 1;

闭包的参数和返回值： 闭包可以有零个或多个参数，并且可以返回一个值。

let calculate = |a, b, c| a * b + c;

闭包的调用：闭包可以像函数一样被调用。

let result = calculate(1, 2, 3);
*/
mod traitBounds;
fn main() {
    println!("Hello, world!");
    // anonymous_function();
    /*let b = 10;
    traitBounds::apply(
        |x| {
            return x * b;
        },
        32,
    );*/
    // move_anonymous_function();

    let add_five = traitBounds::applyReturn(
        |x| {
            return x * x;
        },
        32,
    );
    println!("{}", add_five(10));
}
//匿名函数
//闭包在 Rust 中类似于匿名函数，可以在代码中以 {} 语法块的形式定义，使用 || 符号来表示参数列表，实例如下
fn anonymous_function() {
    let add = |a: i32, b: i32| a + b;
    println!("{}", add(2, 3));
    //捕获外部变量
    //闭包可以捕获周围环境中的变量，这意味着它可以访问定义闭包时所在作用域中的变量。例如：
    let x = 5;
    let square = |num| num * x;
    println!("{}", square(3)); // 输出: 15
}
//移动与借用
fn move_anonymous_function() {
    let mut data = vec![1, 2, 3];

    // 使用 move 关键字来强制闭包获取所有权
    let mut closure = move || {
        data.push(4);
    };

    closure();

    // println!("{:?}", data); // 编译出错 后续不能使用data，因为data已经被移动，闭包获取了所有权，所以不能再使用
}
//闭包和错误处理
//闭包可以返回 Result 或 Option 类型，并且可以处理错误。
fn find_first_positive(nums: &[i32], is_positive: impl Fn(i32) -> bool) -> Option<usize> {
    nums.iter().position(|&x| is_positive(x))
}

//闭包和多线程
/// 创建并启动多个线程，每个线程将一个数字乘以2
fn threade_closure() {
    use std::thread;

    // 初始化一个包含整数的向量
    let nums = vec![1, 2, 3, 4, 5];

    // 将nums中的每个元素映射为一个新线程，每个线程将执行num * 2的操作
    // 线程是通过move闭包创建的，确保了对nums的元素的所有权转移，以便在线程中使用
    let handles = nums
        .into_iter()
        .map(|num| thread::spawn(move || num * 2))
        .collect::<Vec<_>>();

    // 遍历所有线程句柄，等待每个线程完成，并打印结果
    for handle in handles {
        // join方法用于等待线程完成，并返回结果
        let result = handle.join().unwrap();
        println!("Result: {}", result);
    }
}
