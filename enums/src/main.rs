enum IpAddrKind {
    V4(String),
    V6(String),
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn main() {
    let four = IpAddrKind::V4(String::from("127.0.0.1"));
    let six = IpAddrKind::V6(String::from("::1"));
    value_in_cents(Coin::Nickel);
}

fn value_in_cents(coin: Coin) {
    match coin {
        Coin::Penny => {
            println!("1 cent");
        }
        Coin::Nickel => {
            println!("5 cents");
        }
        Coin::Dime => {
            println!("10 cents");
        }
        Coin::Quarter => {
            println!("25 cents");
        }
    }

}
