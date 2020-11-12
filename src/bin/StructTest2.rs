fn main() {
    #[derive(Default)]
    struct Point3d {
        x: i32,
        y: i32,
        z: i32,
    }

    let origin = Point3d::default();
    println!("x:{}", origin.x);
    let point = Point3d { y: 1, ..origin };
    println!("x:{},y:{},z:{}", point.x, point.y, point.z);
    //这个是什么鬼，完全看不懂
    let Point3d { x: x0, y: y0, .. } = point;
}