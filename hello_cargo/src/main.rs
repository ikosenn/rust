mod file_closure;

fn main() {
    println!("Hello, world!");

    // test array ownership
    let a = [String::new()];
    let b = a;
    println!("{:?}", b);

    // test
    let s1 = String::from("Hello, ");
    let mut s2 = String::from("world!");
    let s3 = s1 + &s2;
    s2 = String::from("John");
    println!("string is {s2}...{s3}");
    file_closure::read_or_create("/tmp/new.txt");
}
