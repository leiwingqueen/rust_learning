/*
数组 array
Rust 使用数组存储相同类型的数据集。 [T; N]表示一个拥有 T 类型，N 个元素的数组。数组的大小是固定。
 */
fn main() {
    let mut array: [i32; 3] = [1; 3];
    //let mut array: [i32; 3] = [0,3,2];

    //array[1] = 1;
    //array[2] = 2;

    //assert_eq!([1, 2], &array[1..]);

    // This loop prints: 0 1 2
    for x in &array {
        println!("{} ", x);
    }
}