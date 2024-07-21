/*
Rust 集合与字符串
集合（Collection）是数据结构中最普遍的数据存放形式，
Rust 标准库中提供了丰富的集合类型帮助开发者处理数据结构的操作。
*/
mod collection;
mod map;
mod string;
fn main() {
    println!("Hello, world!");
    // collection::vector();
    // collection::traversal_vec();
    // collection::traversal_vec_plus();
    // string::contain_utf_char_string();
    // string::get_one_char_by_string();
    map::map_example_1();
}
