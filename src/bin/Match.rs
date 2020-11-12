fn main() {
    //这个是不是有点类似case的语法
    let day = 10;
    match day {
        0 | 6 => println!("weekend"),
        1...5 => println!("weekday"),
        _ => println!("invalid"),
    }
}