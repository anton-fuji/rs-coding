use proconio::{fastout, input};
use std::i64::MAX;

const L: i64 = 9999;
#[fastout]
fn main() {
    input! {
      n: i64,
      (a, b, c): (i64, i64, i64),
    }

    let mut res = MAX;
    for i in 0..=L {
        for j in 0..(L - i) {
            let r = n - a * i - b * j;
            if r < 0 {
                continue;
            }
            if r % c != 0 {
                continue;
            }
            let nc = r / c;
            res = res.min(i + j + nc);
        }
    }
    println!("{res}");
}
