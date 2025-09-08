#![allow(unused_imports)]
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        t: usize,
        cases: [(i64, i64, i64); t],
    }

    for (n_a, n_b, n_c) in cases {
        let total = n_a + n_b + n_c;
        let res = std::cmp::min(n_a, std::cmp::min(n_c, total / 3));
        println!("{res}");
    }
}
