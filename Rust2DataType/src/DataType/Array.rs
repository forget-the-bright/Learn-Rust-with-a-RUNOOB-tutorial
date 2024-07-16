//数组用一对 [ ] 包括的同类型数据。
use std::any::type_name_of_val;
pub fn printArray() {
    let a = [1, 2, 3, 4, 5];
    let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];
    println!("a: {:?} | type {}", a, type_name_of_val(&a));
    println!("months: {:?} | type {}", months, type_name_of_val(&months));
}

pub fn printArray2() {
    let a = [1, 2, 3, 4, 5];
    // a 是一个长度为 5 的整型数组

    let b = ["January", "February", "March"];
    // b 是一个长度为 3 的字符串数组

    let c: [i32; 5] = [1, 2, 3, 4, 5];
    // c 是一个长度为 5 的 i32 数组

    let d = [3; 5];
    // 等同于 let d = [3, 3, 3, 3, 3];

    let first = a[0];
    let second = a[1];
    // 数组访问

    //a[0] = 123; // 错误：数组 a 不可变
    let mut a = [1, 2, 3]; //mut修饰后可变
    a[0] = 4; // 正确
}
