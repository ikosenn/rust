fn main() {
    println!("Hello, world!");

    // test array ownership
    let a = [String::new()];
    let b = a;
    println!("{:?}", b);
}
