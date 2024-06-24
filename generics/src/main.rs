use std::fmt::Display;

#[derive(Debug)]
struct Point<T, U> {
    x: T,
    y: U
}

impl<T,U> Point<T, U> {
    fn x(&self) -> &T {
        &self.x
    }
}

impl Point<f64, f64> {
    fn dist_from_origin(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

fn largest<T: Display + PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}


fn longest_lifetimes<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s1.len() > s2.len() {
        s1
    } else {
        s2
    }

}

//struct

struct ImportantExcerpt<'a> {
    part: &'a str,
}


impl<'a> ImportantExcerpt<'a> {
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {announcement}");
        self.part
    }
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest(&number_list);
    println!("The largest number is {result}");

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {result}");

    // structs
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };
    let mixed = Point { x: 1, y: 4.0 };
    println!("Integer: {integer:?}, float: {float:?}");

    let str1 = "Hello boom";
    let str2 = "World";
    let str_lngest = longest_lifetimes(str2, str1);

    println!("Longest is {str_lngest}");

    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };
    println!("{}",i.announce_and_return_part("test"));
}
