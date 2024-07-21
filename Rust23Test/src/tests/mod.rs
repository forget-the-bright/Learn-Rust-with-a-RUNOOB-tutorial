use super::*;

/*

在这个例子中，test_example 是一个测试函数，它使用了 assert_eq! 宏来检查一个条件是否为真。
要运行测试，可以在命令行中使用 cargo test 命令
        cargo test
        cargo test  -- tests::test_example
        cargo test  -- tests::test_example --exact --show-output
*/
#[test]
fn test_example() {
    assert_eq!(2 + 2, 4);
}
/*
测试技巧
使用 assert!、assert_eq! 和 assert_ne! 宏来验证条件。
使用 should_panic 属性来测试代码是否如预期那样恐慌（panic）。
使用 #[ignore] 属性来标记暂时不想运行的测试。
使用 cargo test -- --ignored 来运行被忽略的测试。
使用 cargo test -- --test-threads=1 来串行运行测试，这对于调试测试中的并发问题很有帮助
*/

/*
这个文档测试方法只能在lib crate中使用
binary crate 中无法使用
执行命令
          cargo.exe test --doc -- tests::add_one
          cargo.exe test --doc -- tests::add_one --show-output
          cargo.exe test --doc -- tests::add_one --show-output --exact

*/
/// Adds one to the number given.
///
/// # Examples
///
/// ```
/// let five = 5;
///
/// assert_eq!(6, add_one(five));
/// ```
pub fn add_one(x: i32) -> i32 {
    x + 1
}
