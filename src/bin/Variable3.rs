/*
let解构
为什么在 Rust 里面声明一个变量的时候要采用 let 绑定表达式？ 那是因为 let 绑定表达式的表达能力更强，而且 let 表达式实际上是一种模式匹配。
 */
fn main() {
    //let (a, mut b): (bool, bool) = (true, false);
    let (a, mut b) = (true, false);
    println!("a = {:?}, b = {:?}", a, b);
    //a 不可变绑定
    //a = false;

    //b 可变绑定
    b = true;
    assert_eq!(a, b);
}