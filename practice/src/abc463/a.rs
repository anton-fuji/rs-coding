use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
        x: usize,
        y: usize,
    }

    let res = 9 * x == 16 * y;
    println!("{}", if res { "Yes" } else { "No" });
}
