/*
async 关键字
async 关键字用于定义异步函数，即返回 Future 或 impl Future 类型的函数。异步函数执行时会返回一个未完成的 Future 对象，它表示一个尚未完成的计算或操作。
异步函数可以包含 await 表达式，用于等待其他异步操作的完成。
*/
async fn hello() -> String {
    "Hello, world!".to_string()
}
/*
await 关键字
await 关键字用于等待异步操作的完成，并获取其结果。
await 表达式只能在异步函数或异步块中使用，它会暂停当前的异步函数执行，等待被等待的 Future 完成，然后继续执行后续的代码。
*/
async fn print_hello() {
    let result = hello().await;
    println!("{}", result);
}
/*
异步函数返回值
异步函数的返回值类型通常是 impl Future<Output = T>，其中 T 是异步操作的结果类型。
由于异步函数的返回值是一个 Future，因此可以使用 .await 来等待异步操作的完成，并获取其结果。
*/
async fn add(a: i32, b: i32) -> i32 {
    a + b
}
/*
异步块
除了定义异步函数外，Rust 还提供了异步块的语法，可以在同步代码中使用异步操作。
异步块由 async { } 构成，其中可以包含异步函数调用和 await 表达式。
*/
pub fn sync_contain_async_block() {
    let result = async {
        let result1 = hello().await;
        let result2 = add(1, 2).await;
        println!("Result: {}, {}", result1, result2);
    };
}
/*
异步任务执行
在 Rust 中，异步任务通常需要在执行上下文中运行，
可以使用 tokio::main、async-std 的 task::block_on 或 futures::executor::block_on 等函数来执行异步任务。
这些函数会接受一个异步函数或异步块，并在当前线程或执行环境中执行它。
*/
use async_std::task;
pub fn run_async_task() {
    let result = async {
        let result1 = hello().await;
        let result2 = add(1, 2).await;
        println!("Result: {}, {}", result1, result2);
    };
    task::block_on(result);
}

/*
错误处理
await 后面跟一个 ? 操作符可以传播错误。
如果 await 的 Future 完成时返回了一个错误，那么这个错误会被传播到调用者。

*/
struct MyType {
    // ...
}
// 定义一个自定义错误类型
#[derive(Debug)]
struct MyError {
    message: String,
}
impl MyError {
    fn new(message: &str) -> Self {
        MyError {
            message: message.to_string(),
        }
    }
}
async fn some_async_operation() -> Result<(), MyError> {
    // 模拟一个可能失败的操作
    let result = false;
    add(1, 2).await;
    if result {
        Ok(())
    } else {
        Err(MyError::new("Something went wrong"))
    }
}
async fn my_async_function() -> Result<(), MyError> {
    some_async_operation().await?;
    Ok(())
    // 如果 some_async_operation 出错，错误会被传播
}
/*
异步 trait 方法
Rust 允许为 trait 定义异步方法。这使得你可以为不同类型的对象定义异步操作。
*/

trait MyAsyncTrait {
    async fn async_method(&self) -> Result<(), MyError>;
}

impl MyAsyncTrait for MyType {
    async fn async_method(&self) -> Result<(), MyError> {
        // 异步逻辑
        if 1 == 2 {
            Ok(())
        } else {
            Err(MyError::new("Something went wrong"))
        }
    }
}

/*
异步上下文
在 Rust 中，异步代码通常在异步运行时（如 Tokio 或 async-std）中执行。这些运行时提供了调度和执行异步任务的机制。

#[tokio::main]
async fn main() {
    some_async_operation().await;
}

以上代码中，#[tokio::main] 属性宏将 main 函数包装在一个异步运行时中。
*/

/*
异步宏
Rust 提供了一些异步宏，如 tokio::spawn，用于在异步运行时中启动新的异步任务。
*/

pub async fn macros_async() {
    let handle = tokio::spawn(async {
        // 异步逻辑
    });
    handle.await.unwrap();
}
/*
异步 I/O
Rust 的标准库提供了异步 I/O 操作，如 tokio::fs::File 和 async_std::fs::File。
*/
use std::path::Path;
use tokio::fs::File;
use tokio::io::{self, AsyncReadExt};
// 提前讲解下宏的定义
macro_rules! path {
    // 模式匹配
    ($name:expr) => {
        // 宏的展开
        Path::new($name)
    };
}
pub async fn io_async() -> io::Result<()> {
    let mut file = File::open(path!("Cargo.toml.bak")).await?;
    let mut contents = String::new();
    file.read_to_string(&mut contents).await?;
    println!("Contents: {}", contents);
    Ok(())
}
/*
异步通道
Rust 的一些异步运行时提供了异步通道（如 tokio::sync::mpsc），允许在异步任务之间传递消息。
*/
use tokio::spawn;
use tokio::sync::mpsc;
pub async fn channnel_async() {
    let (tx, mut rx) = mpsc::channel(32);

    let child = spawn(async move {
        let response = "Hello, world!".to_string();
        tx.send(response).await.unwrap();
    });

    let response = rx.recv().await.unwrap();
    println!("Received: {}", response);

    child.await.unwrap();
}

/*
总结
Rust 的异步编程模型 async/await 提供了一种简洁、高效的方式来处理异步操作。

它允许开发者以一种更自然和直观的方式来处理异步操作，同时保持了 Rust 的安全性和性能。

通过 async/await，Rust 为异步编程提供了一流的语言支持，使得编写高效且可读性强的异步程序变得更加容易。
*/
