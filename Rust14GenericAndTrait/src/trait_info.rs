/*
特性
特性（trait）概念接近于 Java 中的接口（Interface），但两者不完全相同。
特性与接口相同的地方在于它们都是一种行为规范，可以用于标识哪些类有哪些方法。

特性在 Rust 中用 trait 表示：
*/
trait Descriptive {
    fn describe(&self) -> String;
}
/*
Descriptive 规定了实现者必需有 describe(&self) -> String 方法。

我们用它实现一个结构体：
*/
struct Person {
    name: String,
    age: u8,
}

impl Descriptive for Person {
    fn describe(&self) -> String {
        format!("{} {}", self.name, self.age)
    }
}
/*
格式是：

impl <特性名> for <所实现的类型名>

Rust 同一个类可以实现多个特性，每个 impl 块只能实现一个。
*/

//=============================================================默认特性===================================================================//

/*
默认特性
这是特性与接口的不同点：接口只能规范方法而不能定义方法，但特性可以定义方法作为默认方法，
因为是"默认"，所以对象既可以重新定义方法，也可以不重新定义方法使用默认的方法
*/
pub fn default_trait() {
    trait Descriptive {
        fn describe(&self) -> String {
            String::from("[Object]")
        }
    }

    struct Person {
        name: String,
        age: u8,
    }

    impl Descriptive for Person {
        fn describe(&self) -> String {
            format!("{} {}", self.name, self.age)
        }
    }

    let cali = Person {
        name: String::from("Cali"),
        age: 24,
    };
    println!("{}", cali.describe());

    /*
    运行结果：
        Cali 24
    如果我们将 impl Descriptive for Person 块中的内容去掉，那么运行结果就是：
        [Object]
     */
}

//==========================================================特性做参数====================================================================//

/*
特性做参数
很多情况下我们需要传递一个函数做参数，例如回调函数、设置按钮事件等。在 Java 中函数必须以接口实现的类实例来传递，在 Rust 中可以通过传递特性参数来实现
*/
fn output1(object: impl Descriptive) {
    println!("{}", object.describe());
}
/*
任何实现了 Descriptive 特性的对象都可以作为这个函数的参数，这个函数没必要了解传入对象有没有其他属性或方法，
只需要了解它一定有 Descriptive 特性规范的方法就可以了。当然，此函数内也无法使用其他的属性与方法。
特性参数还可以用这种等效语法实现：
*/
fn output2<T: Descriptive>(object: T) {
    println!("{}", object.describe());
}
//这是一种风格类似泛型的语法糖，这种语法糖在有多个参数类型均是特性的情况下十分实用：
fn output2_<T: Descriptive>(arg1: T, arg2: T) {
    println!("{}", arg1.describe());
    println!("{}", arg2.describe());
}

//=============================================================多个特性 + =====================================================================//
trait Summary {
    fn summary(&self) -> String;
}
trait Display {
    fn show(&self) -> String;
}
fn notify1(item: impl Summary + Display) {
    println!("{}", item.summary());
}
fn notify2<T: Summary + Display>(item: T) {}
/*
注意：仅用于表示类型的时候，并不意味着可以在 impl 块中使用。

复杂的实现关系可以使用 where 关键字简化，例如：
*/
use std::fmt::Debug;
fn some_function1<T: Display + Clone, U: Clone + Debug>(t: T, u: U) {}
//可以简化成：

fn some_function2<T, U>(t: T, u: U)
where
    T: Display + Clone,
    U: Clone + Debug,
{
}
//在了解这个语法之后，泛型章节中的"取最大值"案例就可以真正实现了
pub fn multi_trait() {
    trait Comparable {
        fn compare(&self, object: &Self) -> i8;
    }

    fn max<T: Comparable>(array: &[T]) -> &T {
        let mut max_index = 0;
        let mut i = 1;
        while i < array.len() {
            if array[i].compare(&array[max_index]) > 0 {
                max_index = i;
            }
            i += 1;
        }
        &array[max_index]
    }

    impl Comparable for f64 {
        fn compare(&self, object: &f64) -> i8 {
            if &self > &object {
                1
            } else if &self == &object {
                0
            } else {
                -1
            }
        }
    }
    impl Comparable for i32 {
        fn compare(&self, object: &i32) -> i8 {
            if &self > &object {
                1
            } else if &self == &object {
                0
            } else {
                -1
            }
        }
    }
    let arr = [1.0, 3.0, 5.0, 4.0, 2.0];
    println!("maximum of arr is {}", max(&arr));
    let arr = [1, 3, 5, 4, 2];
    println!("maximum of arr is {}", max(&arr));
}
