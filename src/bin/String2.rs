/*
String
String 是一个带有的 vec:Vec<u8> 成员的结构体，你可以理解为 str 类型的动态形式。 它们的关系相当于 [T] 和 Vec<T> 的关系。 显然 String 类型也有压入和弹出。
 */
fn main() {
    // 创建一个空的字符串
    let mut s = String::new();
// 从 `&str` 类型转化成 `String` 类型
    let mut hello = String::from("Hello, ");
// 压入字符和压入字符串切片
    hello.push('w');
    hello.push_str("orld!");

// 弹出字符。
    let mut s = String::from("foo");
    assert_eq!(s.pop(), Some('o'));
    assert_eq!(s.pop(), Some('o'));
    assert_eq!(s.pop(), Some('f'));
    assert_eq!(s.pop(), None);
}