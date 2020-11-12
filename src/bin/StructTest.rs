/*
结构体
结构体 (struct) 是一种记录类型，所包含的每个域 (field) 都有一个名称。 每个结构体也都有一个名称，通常以大写字母开头，使用驼峰命名法。 元组结构体 (tuple struct) 是由元组和结构体混合构成，元组结构体有名称， 但是它的域没有。当元组结构体只有一个域时，称为新类型 (newtype)。 没有任何域的结构体，称为类单元结构体 (unit-like struct)。 结构体中的值默认是不可变的，需要给结构体加上mut使其可变。
 */

// structs
struct Point {
    x: i32,
    y: i32,
}

// tuple structs
struct Color(u8, u8, u8);

// A tuple struct’s constructors can be used as functions.
struct Digit(i32);

// unit-like structs
struct EmptyStruct;

// newtype: a tuple struct with only one element
struct Inches(i32);

fn main() {
    let mut point = Point { x: 0, y: 0 };
    point.x = 1;
    println!("{}", point.x);
    let android_green = Color(0xa4, 0xc6, 0x39);
    let Color(red, green, blue) = android_green;

    let v = vec![0, 1, 2];
    let d: Vec<Digit> = v.into_iter().map(Digit).collect();
    let length = Inches(10);

    let Inches(integer_length) = length;
    let empty = EmptyStruct;
}