use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        s:String,
    }

    let s_char: Vec<char> = s.chars().collect();
    let v1 = s_char[0].to_string();
    let v2 = s_char[2].to_string();

    let a = v1.parse::<i64>().unwrap();
    let b = v2.parse::<i64>().unwrap();

    println!("{}", a * b);
}
