use proconio::{fastout, input};
use std::cmp::max;

#[fastout]
fn main() {
    input! {
      n: usize,
      m: usize,
     mut b: [isize; n],
     mut w: [isize; m]
    }

    b.sort_by(|a, b| b.cmp(a));
    w.sort_by(|a, b| b.cmp(a));

    let mut res = 0;
    let mut sumb = 0;
    let mut sumw = 0;
    let mut bestw = 0;

    for i in 0..n {
        sumb += b[i];
        if i < m {
            sumw += w[i];
            bestw = max(bestw, sumw);
        }
        res = max(res, sumb + bestw);
    }
    println!("{}", res)
}
