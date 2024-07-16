/*
结构体所有权
结构体必须掌握字段值所有权，因为结构体失效的时候会释放所有字段。

这就是为什么本章的案例中使用了 String 类型而不使用 &str 的原因。

但这不意味着结构体中不定义引用型字段，这需要通过"生命周期"机制来实现。

但现在还难以说明"生命周期"概念，所以只能在后面章节说明。
*/
struct Dog {
    name: String,
    age: i8,
}
pub fn example() {
    let mydog = Dog {
        name: String::from("wangcai"),
        age: 3,
    };
    let str = mydog.name;
    println!("str={}", str);
    /*
       println!("mydog: name={},age={}", mydog.name, mydog.age);
       编译出错value borrowed here after move
       用mydog.name给str赋值时，所有权就move到的str变量。
       打印时引用mydog.name，此时已经不存在，无法再使用
       应该改为
          let str = mydog.name.clone();
       clone()会创建mydog.name的一个副本。
    */
}
