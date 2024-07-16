/*
整数型（Integer）
整数型简称整型，按照比特位长度和有无符号分为以下种类：
位长度	有符号	无符号
8-bit	i8	     u8
16-bit	i16	     u16
32-bit	i32	     u32
64-bit	i64	     u64
128-bit	i128	u128
arch	isize	usize

isize 和 usize 两种整数类型是用来衡量数据大小的，它们的位长度取决于所运行的目标平台，如果是 32 位架构的处理器将使用 32 位位长度整型。

整数的表述方法有以下几种：

进制	                   例
十进制  	              98_222
十六进制                   0xff
八进制	                   0o77
二进制	                0b1111_0000
字节(只能表示 u8 型)	    b'A'
*/
pub fn println_interger() {
    let _int: u32 = 98_222;
    let _int: i32 = -98_222;
    println!("_int: u32: {}  无符号不能表示负数", _int);
    println!("_int: i32: {}  有符号能表示负数", _int);
}
