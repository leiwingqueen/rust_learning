/*
字符串
Rust 里面有两种字符串类型。String 和 str。

&str
str 类型基本上不怎么使用，通常使用 &str 类型，它其实是 [u8] 类型的切片形式 &[u8]。这是一种固定大小的字符串类型。 常见的的字符串字面值就是 &'static str 类型。这是一种带有 'static 生命周期的 &str 类型。
 */
fn main() {
    // 字符串字面值
    let mut hello = "Hello, world!，你好";

    println!("{}",hello);
    hello="ni";

// 附带显式类型标识
    let hello: &'static str = "Hello, world!";
    println!("{}",hello);
}