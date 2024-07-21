/*
字符串
字符串类（String）到本章为止已经使用了很多，所以有很多的方法已经被读者熟知。本章主要介绍字符串的方法和 UTF-8 性质。
新建字符串：
let string = String::new();

*/
//基础类型转换成字符串
pub fn base_type_converted_to_string() {
    let one = 1.to_string(); // 整数到字符串
    let float = 1.3.to_string(); // 浮点数到字符串
    let slice = "slice".to_string(); // 字符串切片到字符串
}
//包含 UTF-8 字符的字符串:
pub fn contain_utf_char_string() {
    let mut vector: Vec<String> = Vec::new();
    vector.push(String::from("السلام عليكم"));
    vector.push(String::from("Dobrý den"));
    vector.push(String::from("Hello"));
    vector.push(String::from("שָׁלוֹם"));
    vector.push(String::from("नमस्ते"));
    vector.push(String::from("こんにちは"));
    vector.push(String::from("안녕하세요"));
    vector.push(String::from("你好"));
    vector.push(String::from("Olá"));
    vector.push(String::from("Здравствуйте"));
    vector.push(String::from("Hola"));
    for string in &mut vector {
        println!("{}", string);
    }
}
//字符串追加
pub fn append_char_string() {
    let mut s = String::from("run");
    s.push_str("oob"); // 追加字符串切片
    s.push('!'); // 追加字符
    println!("appended string: {}", s);
}
//用 + 号拼接字符串
pub fn concat_add_string() {
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2;
    println!("concat_add_string: {}", s3);
}
//这个语法也可以包含字符串切片
pub fn string_slice() {
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = s1 + "-" + &s2 + "-" + &s3;
    println!("string_slice: {}", s);
}
//使用 format! 宏来格式化字符串
pub fn macro_format_string() {
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{}-{}-{}", s1, s2, s3);
    println!("macro_format_string: {}", s);
}
//字符串长度：
pub fn string_length() {
    let s = String::from("hello");
    println!("string_length {} : {}", s, s.len());
    let s = "你好";
    println!("string_length {} : {}", s, s.len());
    /*
    这里 len 的值是 6。因为中文是 UTF-8 编码的，每个字符长 3 字节，所以长度为6。
    但是 Rust 中支持 UTF-8 字符对象，所以如果想统计字符数量可以先取字符串为字符集合
     */
    let s = "hello你好";
    println!("string_length {} : {}", s, s.chars().count());
}

//遍历字符串
pub fn traversal_string() {
    let s = String::from("hello中文");
    for c in s.chars() {
        println!("{}", c);
    }
}
//从字符串中取单个字符
pub fn get_one_char_by_string() {
    let s = String::from("EN中文");
    let a = s.chars().nth(2);
    println!("{:?}", a);
    // 但是请注意此用法有可能肢解一个 UTF-8 字符！那样会报错
    let sub = &s[0..2];
    // let sub = &s[0..3];  这个就会肢解报错
    println!("{}", sub);
}
