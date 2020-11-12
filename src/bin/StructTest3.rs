mod graph {
    #[derive(Default)]
    pub struct Point {
        pub x: i32,
        y: i32,
    }

    //为什么其他模块还是访问不了这个function?
    pub fn inside_fn() {
        let p = Point { x: 1, y: 2 };
        println!("{}, {}", p.x, p.y);
    }
}

fn outside_fn() {
    let p = graph::Point::default();
    println!("{}", p.x);
    //println!("{}", p.y);
    // field `y` of struct `graph::Point` is private
}

fn main() {
    outside_fn();
}