fn main() {
    fn f1(s: String) -> String {
        s + " world!"
    }

    let a = String::from("Hello");
    let b = a;
    let c = f1(b);
    println!("{}", c);
}