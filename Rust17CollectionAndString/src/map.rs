/*
映射表

映射表（Map）在其他语言中广泛存在。其中应用最普遍的就是键值散列映射表（Hash Map）。
新建一个散列值映射表:
*/

use std::collections::HashMap;
//注意：这里没有声明散列表的泛型，是因为 Rust 的自动判断类型机制。
pub fn map_example_1() {
    let mut map = HashMap::new();
    //注意：这里没有声明散列表的泛型，是因为 Rust 的自动判断类型机制。
    map.insert("color", "red");
    map.insert("size", "10 m^2");

    println!("{}", map.get("color").unwrap());
    println!("{:?}", map);
    // map 的遍历
    for p in map.iter() {
        println!("{:?}", p);
    }
    /*
    Rust 的映射表是十分方便的数据结构，
    当使用 insert 方法添加新的键值对的时候，
    如果已经存在相同的键，会直接覆盖对应的值。
    如果你想"安全地插入"，就是在确认当前不存在某个键时才执行的插入动作，可以这样
     */
    map.entry("color").or_insert("red");
    /*
       这句话的意思是如果没有键为 "color" 的键值对就添加它并设定值为 "red"，否则将跳过。

       在已经确定有某个键的情况下如果想直接修改对应的值，有更快的办法
    */
    //这句话的意思是如果没有键为 "color" 的键值对就添加它并设定值为 "green"，否则将跳过
    map.entry("color").or_insert("green");
    println!("{:?}", map);
    //在已经确定有某个键的情况下如果想直接修改对应的值，有更快的办法
    if let Some(x) = map.get_mut(&"color") {
        *x = "green";
    }
    println!("{}", map.get("color").unwrap());
}
