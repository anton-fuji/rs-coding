use num::integer::gcd;
use proconio::{fastout, input};

const MAX: u64 = 1_000_000_000_000_000_000;
#[fastout]
fn main() {
    input! {
      a: u64,
      b: u64,
    }
    let g = gcd(a, b);
    let r = b / g;
    if r > (MAX / a) {
        println!("Large")
    } else {
        println!("{}", a * r)
    }
}
