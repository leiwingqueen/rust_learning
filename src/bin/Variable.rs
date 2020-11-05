fn main() {
    let a1 = 5;
    let a2: i32 = 5;
    assert_eq!(a1, a2);
    //let 绑定 整数变量默认类型推断是 i32

    let b1: u32 = 5;
    //assert_eq!(a1, b1);
    //去掉上面的注释会报错，因为类型不匹配
    //errer: mismatched types

    println!("end");
}
