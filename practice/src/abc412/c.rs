// use std::{cmp::Reverse, collections::BinaryHeap};
// use itertools::Itertools;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
      t: usize,
    }
    for _ in 0..t {
        solve()
    }
}

fn solve() {
    input! {
      mut s: [i64],
    }
    let n = s.len();
    s[1..n - 1].sort();
    const L: usize = 1000;
    let mut dp = vec![-1; L];
    dp[0] = s[0];

    for &s in &s {
        for i in (1..L).rev() {
            if s <= dp[i - 1] * 2 {
                dp[i] = dp[i].max(s);
            }
        }
    }

    for i in 1..L {
        if dp[i] >= s[n - 1] {
            println!("{}", i + 1);
            return;
        }
    }
    println!("-1");
}
