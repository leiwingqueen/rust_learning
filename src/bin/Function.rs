fn main() {
    //常规用法
    fn add_one(x: i32) -> i32 {
        x + 1
    }
    let mut x = add_one(3);
    x = add_one(x);
    println!("{}", x);

    //panic
    fn diverges() -> ! {
        panic!("This function never returns!");
    }
    //let x: i32 = diverges();
    //let y: String = diverges();

    //匿名函数
    let num = 5;
    let plus_num = |x: i32| x + num;
    let x = plus_num(1);
    println!("{}", x);


    let mut num = 5;
    {
        let mut add_num = move |x: i32| num += x;   // 闭包通过move获取了num的所有权
        add_num(5);
        //为啥加了最终还是5的？对象拷贝？
        println!("{}", num);
    }

// 下面的num在被move之后还能继续使用是因为其实现了Copy特性
// 具体可见所有权(Owership)章节
    assert_eq!(5, num);
    println!("{}", num);
}