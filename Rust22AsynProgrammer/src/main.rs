use tokio;
// 主函数
#[tokio::main]
async fn main() {
    async_info::sync_contain_async_block();
    println!("Start executing async task...");
    // 调用异步任务执行函数，并等待其完成
    async_await::execute_async_task().await;
    println!("Async task completed!");

    //async_await::execute_async_fetch_url(); 方法内有新开启的异步运行时，所以在有[tokio::main]时，该方法无法执行
    //[tokio::main]本身就开启了异步运行时，无法在一个异步运行时中开启另一个异步运行时
    // async_await::execute_async_fetch_url();
    // async_info::run_async_task();
    // async_info::io_async().await;
    async_info::channnel_async().await;
}

// fn main() {
//     async_await::execute_async_fetch_url();
// }

mod async_await;
mod async_info;
