/*
引用（Reference）是 C++ 开发者较为熟悉的概念。

如果你熟悉指针的概念，你可以把它看作一种指针。

实质上"引用"是变量的间接访问方式。
*/
use std::any::type_name_of_val;

pub fn example1() {
    /*
    & 运算符可以取变量的"引用"。
    当一个变量的值被引用时，变量本身不会被认定无效。因为"引用"并没有在栈中复制变量的值
     */
    let s1 = String::from("hello");
    let s2 = &s1;
    println!("s1 is {}, s2 is {}", s1, s2);
    println!(
        "s1  type is {}, s2 type is {}",
        type_name_of_val(&s1),
        type_name_of_val(&s2)
    );
}

//函数参数传递的道理一样 变量本身不会被认定无效。因为"引用"并没有在栈中复制变量的值
pub fn example2() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);
}
fn calculate_length(s: &String) -> usize {
    s.len()
}

/*
引用不会获得值的所有权。

引用只能租借（Borrow）值的所有权。

引用本身也是一个类型并具有一个值，这个值记录的是别的值所在的位置，但引用不具有所指值的所有权
*/
pub fn example3() {
    // 这段程序不正确：因为 s2 租借的 s1 已经将所有权移动到 s3，
    // 所以 s2 将无法继续租借使用 s1 的所有权。如果需要使用 s2 使用该值，。
    /*  let s1 = String::from("hello");
    let s2 = &s1;
    let s3 = s1;
    println!("{}", s2); */
    //必须重新租借所有权
    let s1 = String::from("hello");
    let mut s2 = &s1;
    let s3 = s1;
    s2 = &s3; // 重新从 s3 租借所有权
    println!("{}", s2);
}

//可变引用，和不可变引用

pub fn not_variable_and_variable() {
    // 既然引用不具有所有权，即使它租借了所有权，它也只享有使用权（这跟租房子是一个道理）。如果尝试利用租借来的权利来修改数据会被阻止
    /*
        let s1 = String::from("run");
        let s2 = &s1;
        println!("{}", s2);
        s2.push_str("oob"); // 错误，禁止修改租借的值
        println!("{}", s2);
    */
    //当然，也存在一种可变的租借方式，就像你租一个房子，如果物业规定房主可以修改房子结构，房主在租借时也在合同中声明赋予你这种权利，你是可以重新装修房子的
    //这段程序就没有问题了。我们用 &mut 修饰可变的引用类型
    let mut s1 = String::from("run");
    // s1 是可变的

    let s2 = &mut s1;
    // s2 是可变的引用

    s2.push_str("oob");
    println!("{}", s2);

    //可变引用与不可变引用相比除了权限不同以外，可变引用不允许多重引用，但不可变引用可以
    /*
       let mut s = String::from("hello");

       let r1 = &mut s;
       let r2 = &mut s;

        println!("{}, {}", r1, r2);

    */
    let s = String::from("hello");

    let r1 = &s;
    let r2 = &s;

    println!("不可变引用可以多重引用 {}, {}", r1, r2);
}
//垂悬引用（Dangling References）
/*
这是一个换了个名字的概念，如果放在有指针概念的编程语言里它就指的是那种没有实际指向一个真正能访问的数据的指针
（注意，不一定是空指针，还有可能是已经释放的资源）。它们就像失去悬挂物体的绳子，所以叫"垂悬引用"。
"垂悬引用"在 Rust 语言里不允许出现，如果有，编译器会发现它。
*/
pub fn dangling_references() {
    // let reference_to_nothing = dangle();
}

/* fn dangle() -> &String {
    let s = String::from("hello");
    &s
    //伴随着 dangle 函数的结束，其局部变量的值本身没有被当作返回值，被释放了。
    //但它的引用却被返回，这个引用所指向的值已经不能确定的存在，故不允许其出现。
} */
