/*
Rust 中的迭代器是一种方便、高效的数据遍历方法，它提供了一种抽象的方式来访问集合中的每个元素，而不需要显式地管理索引或循环。

迭代器允许你以一种声明式的方式来遍历序列，如数组、切片、链表等集合类型的元素。

迭代器背后的核心思想是将数据处理过程与数据本身分离，使代码更清晰、更易读、更易维护。

迭代器遵循以下原则：

1.惰性求值：迭代器不会立即计算其元素，而是在需要时才计算，这使得迭代器可以用于处理无限序列。
 例如，当调用 map() 或 filter() 方法时，并不会立即对集合进行转换或过滤，而是返回一个新的迭代器，只有当真正需要获取数据时，才会对数据进行转换或过滤。

2.消费性：在迭代器完成迭代后，它所迭代的集合将被消费，即集合的所有权被转移给迭代器，集合不能再被使用。

3.不可变访问：迭代器默认以不可变方式访问其元素，这意味着在迭代过程中不能修改元素。

4.所有权：迭代器可以处理拥有或借用的元素。当迭代器借用元素时，它不会取得元素的所有权。例如，iter() 方法返回的是一个借用迭代器，
而 into_iter() 方法返回的是一个获取所有权的迭代器。
*/
// 文档查看：https://doc.rust-lang.org/std/iter/trait.Iterator.html
// 迭代器：https://rustwiki.org/zh-CN/book/%E5%AE%98%E6%96%B9%E4%BB%
// 菜鸟教程: https://www.runoob.com/rust/rust-iter.html
fn main() {
    println!("Hello, world!");
    // create_iterator();
    //function_iterator();
    // iterat_iterator();
    // adapter();
    Iterator_chain();
}

//1. 创建迭代器的方式
fn create_iterator() {
    // 通过vector容器创建迭代器
    let vec = vec![1, 2, 3];
    let iter = vec.iter();

    // 使用 iter_mut() 方法创建可变借用迭代器：
    let mut vec = vec![1, 2, 3, 4, 5];
    let iter_mut = vec.iter_mut();

    //使用 into_iter() 方法创建获取所有权的迭代器：
    let vec = vec![1, 2, 3, 4, 5];
    let into_iter = vec.into_iter();
}
//2. 迭代器方法
fn function_iterator() {
    /*
     Rust 的迭代器提供了丰富的方法来处理集合中的元素，其中一些常见的方法包括：
     map()：对每个元素应用给定的转换函数。
     filter()：根据给定的条件过滤集合中的元素。
     fold()：对集合中的元素进行累积处理。
     skip()：跳过指定数量的元素。
     take()：获取指定数量的元素。
     enumerate()：为每个元素提供索引。
     ......
    */
    //使用 map() 方法对每个元素进行转换：
    let vec = vec![1, 2, 3, 4, 5];
    let squared_vec: Vec<i32> = vec.iter().map(|x| x * x).collect();
    squared_vec.iter().for_each(|x| println!("{}", x));
    //使用 filter() 方法根据条件过滤元素：
    squared_vec
        .iter()
        .filter(|&x| (x % 2 == 0))
        .for_each(|x| println!("{}", x));
}

//3. 遍历迭代器
fn iterat_iterator() {
    let vec = vec![1, 2, 3, 4, 5];
    //在这个循环中，vec.iter() 返回一个迭代器，
    //for 循环遍历这个迭代器，并将每个元素赋值给 num 变量，然后执行循环体中的代码。
    for &i in vec.iter() {
        println!("{}", i);
    }
}

//4.消费迭代器
fn into_iter() {
    let arr = vec![1, 2, 3];
    let mut iter = arr.into_iter();
    while let Some(val) = iter.next() {
        println!("{}", val);
    }
}

//5.适配器
fn adapter() {
    let vec = vec![1, 2, 3, 4, 5];
    //迭代器适配器是一系列提供给迭代器的函数，它们可以修改迭代器的行为。例如 map, filter, take 等。
    //使用 collect() 方法将迭代器转换为一个集合：
    let vec2: Vec<i32> = vec.iter().map(|x| x * x).collect();
    println!("{:?}", vec2);
}

//迭代器链
fn Iterator_chain() {
    //可以将多个迭代器适配器链接在一起，形成迭代器链。
    use std::iter::Peekable;

    let arr = [1, 2, 3, 4, 5];
    let mut iter = arr.into_iter().peekable();
    while let Some(val) = iter.next() {
        if val % 2 == 0 {
            continue;
        }
        println!("{}", val);
    }
}
