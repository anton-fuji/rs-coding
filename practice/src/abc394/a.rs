use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
    s: String,
    }

    for c in s.chars() {
        if c == '2' {
            print!("{c}");
        }
    }
    println!();
}
