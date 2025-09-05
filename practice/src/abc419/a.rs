use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        s: String,
    }

    let ss: &str = &s;
    match ss {
        "red" => println!("SSS"),
        "blue" => println!("FFF"),
        "green" => println!("MMM"),
        _ => println!("Unknown"),
    }
}
