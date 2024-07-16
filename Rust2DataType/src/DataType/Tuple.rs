//元组是用一对 ( ) 包括的一组数据，可以包含不同种类的数据
use std::any::type_name_of_val;
pub fn printTuple() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of y is: {}", y);
    println!("The value of z is: {}", z);
    println!("The value of x is: {}", x);
    println!(
        "The value of tup is: {:?} | tup type {}",
        tup,
        type_name_of_val(&tup)
    );
}
