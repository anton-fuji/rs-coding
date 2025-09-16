use std::usize;

use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
      n: usize,
      s: [Chars; n],
      t: [Chars; n],
    }

    let mut res = usize::MAX;
    for k in 0..4 {
        let mut count = k;
        for i in 0..n {
            for j in 0..n {
                let (mut si, mut sj) = (i, j);
                for _ in 0..k {
                    (si, sj) = (n - 1 - sj, si);
                }
                count += if s[si][sj] != t[i][j] { 1 } else { 0 };
            }
        }
        res = res.min(count);
    }
    println!("{res}");
}
