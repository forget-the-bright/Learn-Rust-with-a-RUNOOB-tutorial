/*

Rust 很多地方受 JavaScript 影响，在实例化结构体的时候用 JSON 对象的 key: value 语法来实现定义
let runoob = defineStruct::Site {
        domain: String::from("www.runoob.com"),
        name: String::from("RUNOOB"),
        nation: String::from("China"),
        found: 2013,
        };
格式： 结构体类名 {
        字段名 : 字段值,
        ...
     }


输出结构体
调试中，完整地显示出一个结构体实例是非常有用的。但如果我们手动的书写一个格式会非常的不方便。所以 Rust 提供了一个方便地输出一整个结构体的方法
导入调试库 #[derive(Debug)] ，之后在 println 和 print 宏中就可以用 {:?} 占位符输出一整个结构体
    println!("Hello, Struct! {:?}", runoob);
输出： Hello, Struct! Site { domain: "www.runoob.com", name: "RUNOOB", nation: "China", found: 2013 }

格式化输出的使用这个
    println!("Hello, Struct! {:#?}", runoob);
输出 ：Hello, Struct! Site {
    domain: "www.runoob.com",
    name: "RUNOOB",
    nation: "China",
    found: 2013,
}
*/
//结构体所有权
pub mod ownership;
//元组结构体
pub mod tupleStruct;
//结构体方法
pub mod structMethod;
#[derive(Debug)]
pub struct Site {
    pub domain: String,
    pub name: String,
    pub nation: String,
    pub found: u32,
}

pub fn print_struct() {
    let runoob = Site {
        domain: String::from("www.runoob.com"),
        name: String::from("RUNOOB"),
        nation: String::from("China"),
        found: 2013,
    };
    println!("Hello, Struct! {:?}", runoob);
    println!("Hello, Struct! {:#?}", runoob);
}

pub fn print_struct2() {
    let domain = String::from("www.runoob.com");
    let name = String::from("RUNOOB");
    //如果正在实例化的结构体有字段名称和现存变量名称一样的，可以简化书写
    let runoob = Site {
        domain, // 等同于 domain : domain,
        name,   // 等同于 name : name,
        nation: String::from("China"),
        found: 2013,
    };
    println!("Hello, Struct runoob! {:#?}", runoob);

    // 有这样一种情况：你想要新建一个结构体的实例，其中大部分属性需要被设置成与现存的一个结构体属性一样，
    // 仅需更改其中的一两个字段的值，可以使用结构体更新语法
    let site = Site {
        domain: String::from("www.runoob.com"),
        name: String::from("RUNOOB"),
        ..runoob
    };
    //注意 attention ! ! ! ：..runoob 后面不可以有逗号。这种语法不允许一成不变的复制另一个结构体实例，意思就是说至少重新设定一个字段的值才能引用其他实例的值。
    println!("Hello, Struct site! {:#?}", site);
}
