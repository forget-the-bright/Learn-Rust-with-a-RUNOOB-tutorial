/*
向量

向量（Vector）是一个存放多值的单数据结构，该结构将相同类型的值线性的存放在内存中。
向量是线性表，在 Rust 中的表示是 Vec<T>。
向量的使用方式类似于列表（List），我们可以通过这种方式创建指定类型的向量：

let vector: Vec<i32> = Vec::new(); // 创建类型为 i32 的空向量
let vector = vec![1, 2, 4, 8];     // 通过数组创建向量

我们使用线性表常常会用到追加的操作，但是追加和栈的 push 操作本质是一样的，
所以向量只有 push 方法来追加单个元素
*/

pub fn vector() {
    let mut vector: Vec<i32> = Vec::new();
    vector.push(1);
    vector.push(2);
    vector.push(4);
    vector.push(8);
    println!("{:?}", vector);

    //append 方法用于将一个向量拼接到另一个向量的尾部：
    let mut v2: Vec<i32> = vec![16, 32, 64];
    vector.append(&mut v2);
    println!("{:?}", vector);

    let val = match vector.get(0) {
        Some(value) => value.to_string(),
        None => "".to_string(),
    };
    println!("vector.get(0) : {}", val);

    /*
    因为向量的长度无法从逻辑上推断，get 方法无法保证一定取到值，所以 get 方法的返回值是 Option 枚举类，有可能为空。
    这是一种安全的取值方法，但是书写起来有些麻烦。如果你能够保证取值的下标不会超出向量下标取值范围，你也可以使用数组取值语法
     */
    println!("vector[1]: {}", vector[1]);
    //println!("vector[7]: {}", vector[7]); 越界访问会出错
}
//遍历向量
pub fn traversal_vec() {
    let v = vec![100, 32, 57];
    for i in &v {
        println!("{}", i);
    }
}
//遍历过程中需要更改变量的值
pub fn traversal_vec_plus() {
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
        //i = i + 50; 错误写法 需要解引用在赋值。
    }
    println!("{:?}", v);
}
