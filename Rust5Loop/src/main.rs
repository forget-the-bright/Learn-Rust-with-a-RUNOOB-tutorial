fn main() {
    println!("Hello, world!");
    while_loop();
    //for_loop();
    //_loop();
}

// #region loop循环

/*
我们经常会在一个 while (true) 循环体里实现中途退出循环的操作
Rust 语言有原生的无限循环结构 —— loop
loop 循环可以通过 break 关键字类似于 return 一样使整个循环退出并给予外部一个返回值。
这是一个十分巧妙的设计，因为 loop 这样的循环常被用来当作查找工具使用，如果找到了某个东西当然要将这个结果交出去：
*/
fn _loop() {
    let s = ['R', 'U', 'N', 'O', 'O', 'B'];
    let mut i: i32 = 0;
    use std::ops::Add;
    use std::ops::Sub;
    loop {
        //Rust 不能使用 ++ -- 这样的运算符，只能使用 += 1 等价运算符。
        //或者，Rust 提供了 i.add(1) 和 i.sub(1) 方法，这些方法提供了类似 ++ 和 – 的功能。

        let ch = s[i as usize];
        if ch == 'O' {
            break;
        }
        println!("\'{}\'", ch);
        i = i.add(1);
    }
}
// #endregion

// #region for循环

//for循环
// for 循环在 Rust 中使用 for 循环来迭代集合，集合可以是数组、字符串、向量、集合、字典等。
fn for_loop() {
    let arr = [1, 2, 3, 4, 5];
    for i in arr {
        println!("i = {}", i);
    }
    // 迭代器遍历
    // iter() 方法返回一个迭代器，迭代器可以迭代数组、字符串、向量、集合、字典等。
    for i in arr.iter() {
        println!("i = {}", i);
    }
    //0..5 是一个范围表达式（range expression），用来创建一个不包含上限的开区间，它表示从 0 开始，但不包括 5 的所有整数。这通常用于循环、模式匹配、集合操作等场景。
    //0..5 中的 5 必须是静态的，也就是说，它必须在编译时知道。这通常意味着 5 是一个常量或者一个在编译时确定的值。
    for i in 0..5 {
        println!("a[{}] = {}", i, arr[i]);
    }

    //可以使用 ..= 运算符来创建一个包含上限的闭区间，
    //比如 0..=arr.len() 表示包含数组长度在内的所有整数。 我们要使用 0..=(arr.len() - 1) 进行操作
    //这里稍微麻烦一些，但是必须要判空再进行循环。这种方式只适合于非空数组。
    if !arr.is_empty() {
        for i in 0..=(arr.len() - 1) {
            println!("a[{}] = {}", i, arr[i]);
        }
    }
}

// #endregion

// #region whill循环
/*
while loop 循环
 C 语言中 for 循环使用三元语句控制循环[for (i = 0; i < 10; i++)]， //可以使用0..=arr.len() 来代替。或者使用 while 循环代替
下面是使用 while 循环来代替:
*/
fn while_loop() {
    /*   let mut x = 1;
    while x <= 10 {
        println!("x = {}", x);
        x += 1;
    } */
    //这里推荐通过for去遍历Range
    /*
      在 Rust 中，Range 类型是一个泛型类型，它表示一个数值范围。Range 可以用于迭代器（iterators），模式匹配（pattern matching），以及一些其他用途。
       Range 有几种不同的变体，具体取决于你想要表达的范围类型。以下是一些常见的 Range 类型：
       闭区间：std::ops::Range<T> 闭区间 [a, b] 包括所有的 T 值，使得 a <= x <= b。例如：0..5 表示闭区间 [0, 5]。
       开区间：std::ops::RangeFrom<T> 开区间 (a, b) 包括所有的 T 值，使得 a < x < b。例如：1..5 表示开区间 (1, 5)。
       半开区间：std::ops::RangeTo<T> 半开区间 (a, b) 包括所有的 T 值，使得 a < x <= b。例如：0..5 表示半开区间 (0, 5]。
       单元素区间：std::ops::RangeFull 单元素区间 (a, a) 只包括一个 T 值，即 a。例如：0..0 表示单元素区间 [0, 0]。
       这些 Range 类型都是基于泛型实现的，这意味着你可以为任何类型（如 i32、u8、f64 等）创建 Range。
    */
    let mut iter = 0..5;
    let arr = [1, 2, 3, 4, 5];
    let mut iter = arr.into_iter();
    while let Some(i) = iter.next() {
        println!("i = {}", i);
    }
}
// #endregion
