/*
Rust 文件与 IO
本章介绍 Rust 语言的 I/O 操作
*/
fn main() {
    // get_cmd_args();

    //cmd_line_input();
    // read_file();
    // read_file_bin();
    // read_file_stream();
    //  write_once();
    // write_once_strem();
    // write_append();
    read_write_file();
}
//==========================================命令行===========================================//
/*
接收命令行参数
命令行程序是计算机程序最基础的存在形式，几乎所有的操作系统都支持命令行程序并将可视化程序的运行基于命令行机制。

命令行程序必须能够接收来自命令行环境的参数，这些参数往往在一条命令行的命令之后以空格符分隔。

在很多语言中（如 Java 和 C/C++）环境参数是以主函数的参数（常常是一个字符串数组）传递给程序的，但在 Rust 中主函数是个无参函数，环境参数需要开发者通过 std::env 模块取出，过程十分简单
*/

fn get_cmd_args() {
    let args = std::env::args();
    println!("{:?}", args);
    /*
       也许你得到的结果比这个要长的多，这很正常，
       这个结果中 Args 结构体中有一个 inner 数组，只包含唯一的字符串，代表了当前运行的程序所在的位置。
       但这个数据结构令人难以理解，没关系，我们可以简单地遍历它
    */
    for arg in args {
        println!("{}", arg);
    }
    // 尝试下面的命令进行打印
    // cargo run --package Rust16FileAndIO --bin Rust16FileAndIO first  secend
    /*
       target\debug\Rust16FileAndIO.exe
       first
       secend
    */
}

/*
命令行输入
早期的章节详细讲述了如何使用命令行输出，这是由于语言学习的需要，没有输出是无法调试程序的。
但从命令行获取输入的信息对于一个命令行程序来说依然是相当重要的。
在 Rust 中，std::io 模块提供了标准输入（可认为是命令行输入）的相关功能
*/
fn cmd_line_input() {
    use std::io::stdin;
    let mut str_buf = String::new();
    stdin()
        .read_line(&mut str_buf)
        .expect("Failed to read from stdin");
    println!("Your input line is \n{}", str_buf);
    /*
       std::io::Stdio 包含 read_line 读取方法，
       可以读取一行字符串到缓冲区，返回值都是 Result 枚举类，用于传递读取中出现的错误，
       所以常用 expect 或 unwrap 函数来处理错误。
       注意：目前 Rust 标准库还没有提供直接从命令行读取数字或格式化数据的方法，
       我们可以读取一行字符串并使用字符串识别函数处理数据。
    */
}
//==========================================文件读取===========================================//
/*
文件读取
这是一个将文本文件内容读入字符串的程序
*/
use std::fs;
use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::path::Path;
// 提前讲解下宏的定义
macro_rules! path {
    // 模式匹配
    ($name:expr) => {
        // 宏的展开
        Path::new($name)
    };
}
fn read_file() {
    let text = fs::read_to_string(path!("Cargo.toml")).unwrap();
    println!("{}", text);
}
/*
在 Rust 中读取内存可容纳的一整个文件是一件极度简单的事情，
std::fs 模块中的 read_to_string 方法可以轻松完成文本文件的读取。
但如果要读取的文件是二进制文件，我们可以用 std::fs::read 函数读取 u8 类型集合
*/
fn read_file_bin() {
    let bin = fs::read(path!("Cargo.toml")).unwrap();
    println!("{:?}", bin);
}
/*
以上两种方式是一次性读取，十分适合 Web 应用的开发。
但是对于一些底层程序来说，传统的按流读取的方式依然是无法被取代的，
因为更多情况下文件的大小可能远超内存容量。

Rust 中的文件流读取方式
*/
fn read_file_stream() {
    //每次读取5个字节
    let mut buffer = [0u8; 5];
    //打开文件流
    let mut file = fs::File::open(path!("Cargo.toml")).unwrap();
    loop {
        // 尝试从文件中读取数据到缓冲区中
        let bytes_read = file.read(&mut buffer).unwrap();
        // 如果读取的字节数小于缓冲区大小，表示已经到达文件末尾
        if bytes_read == 0 {
            break;
        }
        // 处理读取的数据，例如打印出来
        println!("读取的数据: {:?}", &buffer[..bytes_read]);

        // 清空缓冲区，准备下一次读取
        buffer = [0u8; 5];
    }
}

//==========================================文件写入===========================================//

/*
文件写入分为一次性写入和流式写入。
流式写入需要打开文件，
打开方式有"新建"（create）和"追加"（append）两种。
一次性写入：
*/
fn write_once() {
    fs::write(path!("Cargo.toml.bak"), b"Hello, world!").unwrap();
    /*
    这和一次性读取一样简单方便。执行程序之后，
    Cargo.toml.bak 文件的内容将会被重写为 Hello, world! 。
    所以，一次性写入请谨慎使用！因为它会直接删除文件内容（无论文件多么大）。
    如果文件不存在就会创建文件。
     */
}
/*
如果想使用流的方式写入文件内容，可以使用 std::fs::File 的 create 方法
*/
fn write_once_strem() {
    /*let mut file = File::create(path!("Cargo.toml.bak")).unwrap();
    file.write_all(b"Hello, world!").unwrap(); */
    let mut file = File::create(path!("Cargo.toml.bak")).unwrap();
    file.write(b"Hello, world!").unwrap();
}

/*
追加写入:

打开的文件一定存放在可变的变量中才能使用 File 的方法！
File 类中不存在 append 静态方法，但是我们可以使用 OpenOptions 来实现用特定方法打开文件
*/
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::io::Result;
fn write_append() {
    let mut file = OpenOptions::new()
        .append(true)
        .open(path!("Cargo.toml.bak"))
        .unwrap();
    file.write(b"\nHello, world!").unwrap();
}

/*
OpenOptions 是一个灵活的打开文件的方法，它可以设置打开权限，
除append 权限以外还有 read 权限和 write 权限，
如果我们想以读写权限打开一个文件可以这样写
*/
fn read_write_file() -> Result<()> {
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .open(path!("Cargo.toml.bak"))?;

    file.write(b"COVER")?;
    Ok(())
}
