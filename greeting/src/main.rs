fn main() {
    let mut x = 5;
    println!("Hello, world!");
    customPrint("Hello 继续");

    let a = 123; // 可以编译，但可能有警告，因为该变量没有被使用
    let a = 456;
}

fn customPrint(value: &str) -> () {
    println!("Hello, world! {0}, {0}", value);
}
