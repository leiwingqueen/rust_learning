fn main() {
    //类似3元表达式
    let x = 5;
    let y = if x == 5 { 10 } else { 15 };
    println!("y:{}", y);

    //尝试按原始的写法
    let i = 5;
    let mut y = 0;
    if x == 5 {
        y = 10;
    } else {
        y = 15;
    }
    println!("y:{}", y);

    let x = 5;
    let z: i32 = if x == 5 { 10 } else { 15 };

    //for语句
    for x in [0, 1, 2, 3].iter() {
        println!("{}", x)
    }
    println!("===================");
    for x in 0..3 {
        println!("{}", x)
    }
    println!("===================");
    //while语句
    let mut x = 0;
    while x < 10 {
        x += 1;
        println!("{}", x)
    }

    //loop循环，这种类似汇编语言的写法，不习惯啊
    'outer: loop {
        println!("Entered the outer loop");
        'inner: loop {
            println!("Entered the inner loop");
            break 'outer;
        }
        println!("This point will never be reached");
    }
    println!("Exited the outer loop");
}