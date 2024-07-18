/*
结构体与枚举类中的泛型

在之前我们学习的 Option 和 Result 枚举类就是泛型的。

Rust 中的结构体和枚举类都可以实现泛型机制。
*/
struct Point1<T> {
    x: T,
    y: T,
}
struct Point2<T1, T2> {
    x: T1,
    y: T2,
}
pub fn struct_example() {
    //这是一个点坐标结构体，T 表示描述点坐标的数字类型。我们可以这样使用：

    let p1 = Point1 { x: 1, y: 2 };
    let p2 = Point1 { x: 1.0, y: 2.0 };
    //使用时并没有声明类型，这里使用的是自动类型机制，但不允许出现类型不匹配的情况如下：

    // let p = Point1 { x: 1, y: 2.0 };
    /*
        x 与 1 绑定时就已经将 T 设定为 i32，所以不允许再出现 f64 的类型。
       如果我们想让 x 与 y 用不同的数据类型表示，可以使用两个泛型标识符
       struct Point2<T1, T2> {
            x: T1,
            y: T2,
        }
    */
    let p = Point2 { x: 1, y: 2.0 };
}
//==========================================================================================================//

/*
在枚举类中表示泛型的方法诸如 Option 和 Result
enum Option<T> {
    Some(T),
    None,
}

enum Result<T, E> {
    Ok(T),
    Err(E),
}
*/
pub fn enum_example() {
    //结构体与枚举类都可以定义方法，那么方法也应该实现泛型的机制，否则泛型的类将无法被有效的方法操作
    struct Point<T> {
        x: T,
        y: T,
    }

    impl<T> Point<T> {
        fn x(&self) -> &T {
            &self.x
        }
    }

    let p = Point { x: 1, y: 2 };
    println!("p.x = {}", p.x());
    //注意，impl 关键字的后方必须有 <T>，因为它后面的 T 是以之为榜样的。但我们也可以为其中的一种泛型添加方法
    impl Point<f64> {
        fn x1(&self) -> f64 {
            self.x
        }
    }
    //impl 块本身的泛型并没有阻碍其内部方法具有泛型的能力：
    impl<T, U> Point2<T, U> {
        fn mixup<V, W>(self, other: Point2<V, W>) -> Point2<T, W> {
            Point2 {
                x: self.x,
                y: other.y,
            }
        }
    }
    //方法 mixup 将一个 Point<T, U> 点的 x 与 Point<V, W> 点的 y 融合成一个类型为 Point<T, W> 的新点。
}
