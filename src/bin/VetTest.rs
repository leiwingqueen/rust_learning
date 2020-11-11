/*
动态数组 Vec
动态数组是一种基于堆内存申请的连续动态数据类型，拥有 O(1) 时间复杂度的索引、压入（push）、弹出（pop)。
 */
fn main() {
    //创建空Vec
    let v: Vec<i32> = Vec::new();
//使用宏创建空Vec
    let v: Vec<i32> = vec![];
//创建包含5个元素的Vec
    let v = vec![1, 2, 3, 4, 5];
//创建十个零
    let v = vec![0; 10];
//创建可变的Vec，并压入元素3
    let mut v = vec![1, 2];
    v.push(3);
//创建拥有两个元素的Vec，并弹出一个元素
    let mut v = vec![1, 2];
    for x in &v {
        println!("v1:{}", x);
    }
    let two = v.pop();
    for x in &v {
        println!("v2:{}", x);
    }
//创建包含三个元素的可变Vec，并索引一个值和修改一个值
    let mut v = vec![1, 2, 3];
    let three = v[2];
    v[1] = v[1] + 5;
    for x in &v {
        println!("v3:{}", x);
    }
}