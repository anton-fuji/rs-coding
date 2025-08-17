use proconio::{input, marker::Chars};

fn main() {
    input! {
      s :Chars,
    }

    match s.iter().collect::<String>().as_str() {
        "red" => println!("SSS"),
        "blue" => println!("FFF"),
        "green" => println!("MMM"),
        _ => println!("Unknown"),
    }
}
