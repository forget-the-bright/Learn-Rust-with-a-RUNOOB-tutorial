use std::any::type_name_of_val;
//布尔型用 bool 表示，值只能为 true 或 false。
pub fn printBool(flag: bool) {
    println!("flag 类型: {}, flag 值: {}", type_name_of_val(&flag), flag);
}
