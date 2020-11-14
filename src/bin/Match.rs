fn main() {
    //这个是不是有点类似case的语法
    let day = 10;
    match day {
        0 | 6 => println!("weekend"),
        1...5 => println!("weekday"),
        _ => println!("invalid"),
    }

    //@标识匹配到的变量(感觉这个用法没啥用)
    let x = 1;
    match x {
        e @ 1...5 => println!("got a range element {}", e),
        _ => println!("anything"),
    }

    //ref关键字。但是这样match关键字的意义在哪里？
    let x = 5;
    let mut y = 5;

    match x {
        // the `r` inside the match has the type `&i32`
        ref r => println!("Got a reference to {}", r),
    }

    match y {
        // the `mr` inside the match has the type `&i32` and is mutable
        ref mut mr => println!("Got a mutable reference to {}", mr),
    }

    //match来解析元祖
    let pair = (0, -2);

    match pair {
        (0, y) => println!("x is `0` and `y` is `{:?}`", y),
        (x, 0) => println!("`x` is `{:?}` and y is `0`", x),
        _ => println!("It doesn't matter what they are"),
    }

    //match来处理struct
    struct Point {
        x: i32,
        y: i32,
    }

    let origin = Point { x: 0, y: 0 };

    match origin {
        Point { x, .. } => println!("x is {}", x),
    }

    enum OptionalInt {
        Value(i32),
        Missing,
    }

    let x = OptionalInt::Value(5);

    match x {
        // 这里是 match 的 if guard 表达式，我们将在以后的章节进行详细介绍
        OptionalInt::Value(i) if i > 5 => println!("Got an int bigger than five!"),
        OptionalInt::Value(..) => println!("Got an int!"),
        OptionalInt::Missing => println!("No such luck."),
    }

    //if let while let
    let number = Some(7);
    let mut optional = Some(0);

// If `let` destructures `number` into `Some(i)`, evaluate the block.
    if let Some(i) = number {
        println!("Matched {:?}!", i);
    } else {
        println!("Didn't match a number!");
    }

// While `let` destructures `optional` into `Some(i)`, evaluate the block.
    while let Some(i) = optional {
        if i > 9 {
            println!("Greater than 9, quit!");
            optional = None;
        } else {
            println!("`i` is `{:?}`. Try again.", i);
            optional = Some(i + 1);
        }
    }
}