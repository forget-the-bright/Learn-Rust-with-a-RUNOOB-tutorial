/*

Rust 异步编程 async/await
在现代编程中，异步编程变得越来越重要，因为它允许程序在等待 I/O 操作（如文件读写、网络通信等）时不被阻塞，从而提高性能和响应性。
异步编程是一种在 Rust 中处理非阻塞操作的方式，允许程序在执行长时间的 I/O 操作时不被阻塞，而是在等待的同时可以执行其他任务。


Rust 提供了多种工具和库来实现异步编程，包括 async 和 await 关键字、futures 和异步运行时（如 tokio、async-std 等），以及其他辅助工具。

        Future：Future 是 Rust 中表示异步操作的抽象。它是一个可能还没有完成的计算，将来某个时刻会返回一个值或一个错误。

        async/await：async 关键字用于定义一个异步函数，它返回一个 Future。await 关键字用于暂停当前 Future 的执行，直到它完成。
*/
//=============================================================== example1 ===============================================================//
// 引入所需的依赖库
use tokio;
// 异步函数，模拟异步任务
async fn async_task() -> u32 {
    // 模拟异步操作，等待 1 秒钟
    tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    // 返回结果
    42
}

// 异步任务执行函数
pub async fn execute_async_task() {
    // 调用异步任务，并等待其完成
    let result = async_task().await;
    // 输出结果
    println!("Async task result: {}", result);
}

//=============================================================== example2 ===============================================================//
// 引入所需的依赖库
use reqwest::get;
use std::error::Error;
use tokio::runtime::Runtime;
// 异步函数，用于执行 HTTP GET 请求并返回响应结果
async fn fetch_url(url: &str) -> Result<String, Box<dyn Error>> {
    // 使用 reqwest 发起异步 HTTP GET 请求
    let response = get(url).await?;
    let body = response.text().await?;
    Ok(body)
}

// 异步任务执行函数
async fn execute_async_task2() -> Result<(), Box<dyn Error>> {
    // 发起异步 HTTP 请求
    let url = "https://www.runoob.com/rust/rust-async-await.html";
    let result = fetch_url(url).await?;
    // 输出响应结果
    println!("Response: {}", result);
    Ok(())
}

pub fn execute_async_fetch_url() {
    /*
    #[tokio::main] 宏已经创建了一个 Tokio 运行时，并且 main 函数是异步的。在异步函数 main 中调用 block_on 是不必要的，并且会导致错误。
     */
    // 创建异步运行时
    let rt = Runtime::new().unwrap();
    // 在异步运行时中执行异步任务
    let result = rt.block_on(execute_async_task2());
    // 处理异步任务执行结果
    match result {
        Ok(_) => println!("Async task executed successfully!"),
        Err(e) => eprintln!("Error: {}", e),
    }
    /*
    以上代码中，我们首先引入了 tokio 和 reqwest 库，分别用于执行异步任务和进行 HTTP 请求。然后定义了一个异步函数 fetch_url，用于执行异步的 HTTP GET 请求，并返回响应结果。

    接着定义了一个异步任务执行函数 execute_async_task，该函数在其中发起了异步 HTTP 请求，并输出响应结果。

    最后，在 main 函数中创建了一个 tokio 异步运行时，并在其中执行了异步任务，处理了异步任务的执行结果。

    运行该程序，可以看到输出了异步 HTTP 请求的响应结果，实例中请求了 JSONPlaceholder 的一个帖子数据，并打印了其内容。
     */
}
