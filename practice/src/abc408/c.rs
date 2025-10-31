use proconio::{fastout, input};
use std::cmp::min;

#[fastout]
fn main() {
    input! {
      n: usize,
      m: usize,
    }

    let mut a = vec![0; n + 2];
    for _ in 0..m {
        input! {
          l: usize,
          r:usize,
        }
        a[l] += 1;
        a[r + 1] -= 1;
    }
    for i in 1..n + 2 {
        a[i] += a[i - 1]
    }
    let mut res = 10000000;
    for i in 1..n + 1 {
        res = min(res, a[i]);
    }
    println!("{}", res);
}
