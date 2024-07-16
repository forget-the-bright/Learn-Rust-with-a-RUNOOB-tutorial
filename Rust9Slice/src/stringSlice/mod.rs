/**
 * 该函数演示了如何将一个字符串分割成两个部分，并将这两个部分与原始字符串一起打印。
 * 它说明了如何使用索引和切片操作来访问和操作字符串的特定部分。
 */
pub fn example() {
    // 创建一个字符串变量`s`，初始化为"broadcast"
    let s = String::from("broadcast");

    //0..5 是一个范围表达式（range expression），用来创建一个不包含上限的开区间（open interval） 也就是不包含5
    // 通过切片操作获取字符串的前五个字符，结果是一个字符串切片
    let part1 = &s[0..5];

    // 5..9 是一个范围表达式，用来创建一个不包含上限的开区间 （open interval） 也就是不包含 9
    // 通过切片操作获取字符串的后四个字符，结果是另一个字符串切片
    let part2 = &s[5..9];

    // 打印原始字符串及其两个切片部分，展示如何拼接字符串
    println!("{}={}+{}", s, part1, part2);
}

//注意 被切片引用的字符串禁止更改其值
fn attention1() {
    let s = String::from("hello world");

    let hello = &s[0..5];
    let world = &s[6..11];

    // 尝试修改 hello 字符串，会报错
    //hello[0] = 'x'; // 错误：字符串不可变
    //s.push_str("yes!"); // 错误
    println!("{} {}", hello, world);
    /*
     s 被部分引用，禁止更改其值。

    实际上，到目前为止你一定疑惑为什么每一次使用字符串都要这样写String::from("hello world") ，直接写 "hello world" 不行吗？

    事已至此我们必须分辨这两者概念的区别了。在 Rust 中有两种常用的字符串类型：str 和 String。

    str 是 Rust 核心语言类型，就是本章一直在讲的字符串切片（String Slice），常常以引用的形式出现（&str）


    凡是用双引号包括的字符串常量整体的类型性质都是 &str：

     let s = "hello";

     这里的 s 就是一个 &str 类型的变量。

     String 类型是 Rust 标准公共库提供的一种数据类型，它的功能更完善——它支持字符串的追加、清空等实用的操作。
     String 和 str 除了同样拥有一个字符开始位置属性和一个字符串长度属性以外还有一个容量（capacity）属性。
     String 和 str 都支持切片，切片的结果是 &str 类型的数据。

     注意：切片结果必须是引用类型，但开发者必须自己明示这一点:
     let slice = &s[0..3];

     有一个快速的办法可以将 String 转换成 &str：
     let s1 = String::from("hello");
     let s2 = &s1[..];

     或者使用 as_str() 方法：
     let s1 = String::from("hello");
     let s2 = s1.as_str();

     或者使用 as_ref() 方法：
       let s1 = String::from("hello");
       let s2: &str = s1.as_ref();
      */
}
