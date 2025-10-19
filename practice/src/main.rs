use std::cmp::min;

use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
      s: usize,
      a: usize,
      b: usize,
      x: usize,
    }

    let mut res = 0;
    res += (x / (a + b)) * a;
    res += min(x % (a + b), a);

    let ans = res * s;
    println!("{ans}");
}
