/*
Rust 并发编程

安全高效的处理并发是 Rust 诞生的目的之一，主要解决的是服务器高负载承受能力。
并发（concurrent）的概念是指程序不同的部分独立执行，这与并行（parallel）的概念容易混淆，并行强调的是"同时执行"。
并发往往会造成并行。
本章讲述与并发相关的编程概念和细节。
*/
fn main() {
    println!("Hello, world!");
    // thread_example_0();
    // thread_example_1();
    // thread_example_2();
    // thread_example_3();
    thread_example_4()
}
/*
线程
线程（thread）是一个程序中独立运行的一个部分。
线程不同于进程（process）的地方是线程是程序以内的概念，程序往往是在一个进程中执行的。
在有操作系统的环境中进程往往被交替地调度得以执行，线程则在进程以内由程序进行调度。
由于线程并发很有可能出现并行的情况，所以在并行中可能遇到的死锁、延宕错误常出现于含有并发机制的程序。
为了解决这些问题，很多其它语言（如 Java、C#）采用特殊的运行时（runtime）软件来协调资源，但这样无疑极大地降低了程序的执行效率。

C/C++ 语言在操作系统的最底层也支持多线程，且语言本身以及其编译器不具备侦察和避免并行错误的能力，这对于开发者来说压力很大，开发者需要花费大量的精力避免发生错误。
Rust 不依靠运行时环境，这一点像 C/C++ 一样。
但 Rust 在语言本身就设计了包括所有权机制在内的手段来尽可能地把最常见的错误消灭在编译阶段，这一点其他语言不具备。
但这不意味着我们编程的时候可以不小心，迄今为止由于并发造成的问题还没有在公共范围内得到完全解决，仍有可能出现错误，并发编程时要尽量小心！

Rust 中通过 std::thread::spawn 函数创建新线程：
*/
use std::thread;
use std::time::Duration;
//方法作为线程执行参数
pub fn thread_example_0() {
    fn spawn_function() {
        for i in 0..5 {
            println!("spawned thread print {}", i);
            thread::sleep(Duration::from_millis(1));
        }
    }
    thread::spawn(spawn_function);

    for i in 0..3 {
        println!("main thread print {}", i);
        thread::sleep(Duration::from_millis(1));
    }
}
//闭包作为线程执行的参数
pub fn thread_example_1() {
    thread::spawn(|| {
        for i in 0..5 {
            println!("spawned thread print {}", i);
            thread::sleep(Duration::from_millis(1));
        }
    });
    for i in 0..3 {
        println!("main thread print {}", i);
        thread::sleep(Duration::from_millis(1));
    }
    //闭包是可以保存进变量或作为参数传递给其他函数的匿名函数。闭包相当于 Rust 中的 Lambda 表达式，格式如下
    /*
         |参数1, 参数2, ...| -> 返回值类型 {
                 // 函数体
         }
    */
}
//join 方法, 等待线程执行完毕，使线程加入到主线程中
pub fn thread_example_2() {
    let handle = thread::spawn(|| {
        for i in 0..5 {
            println!("spawned thread print {}", i);
            thread::sleep(Duration::from_millis(1));
        }
    });
    for i in 0..3 {
        println!("main thread print {}", i);
        thread::sleep(Duration::from_millis(1));
    }
    handle.join().unwrap();
}

//move 强制所有权迁移
pub fn thread_example_3() {
    let s = "move 移交变量所有权到线程中";
    let handle = thread::spawn(move || {
        println!("{}", s);
    });
    handle.join().unwrap();
}
/*
消息传递

Rust 中一个实现消息传递并发的主要工具是通道（channel），
通道有两部分组成，一个发送者（transmitter）和一个接收者（receiver）。
std::sync::mpsc 包含了消息传递的方法
*/
use std::sync::mpsc;
pub fn thread_example_4() {
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
    });
    let received = rx.recv().unwrap();
    println!("Got: {}", received);
    //子线程获得了主线程的发送者 tx，
    //并调用了它的 send 方法发送了一个字符串，然后主线程就通过对应的接收者 rx 接收到了。
}
