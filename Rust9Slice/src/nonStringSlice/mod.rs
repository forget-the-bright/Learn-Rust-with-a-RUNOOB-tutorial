//除了字符串以外，其他一些线性数据结构也支持切片操作，例如数组：
pub fn example() {
    let arr = [1, 3, 5, 7, 9];
    let part = &arr[0..3];
    for i in part.iter() {
        println!("{}", i);
    }
}
