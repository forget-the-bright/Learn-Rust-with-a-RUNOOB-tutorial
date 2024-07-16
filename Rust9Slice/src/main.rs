/*
切片（Slice）是对数据值的部分引用。

切片这个名字往往出现在生物课上，我们做样本玻片的时候要从生物体上获取切片，以供在显微镜上观察。在 Rust 中，切片的意思大致也是这样，只不过它从数据取材引用。
*/
mod nonStringSlice;
mod stringSlice;
fn main() {
    println!("Hello, world!");
    stringSlice::example();
    nonStringSlice::example()
}
