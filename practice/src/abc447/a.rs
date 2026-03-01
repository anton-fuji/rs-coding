use proconio::{fastout, input, marker::Chars};

#[fastout]
#[allow(unused_imports)]
fn main() {
    input! {
        n: usize,
        m: usize
    }
    println!("{}", if n >= 2 * m - 1 { "Yes" } else { "No" })
}
