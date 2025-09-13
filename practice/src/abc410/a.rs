// use std::{cmp::Reverse, collections::BinaryHeap};
// use itertools::Itertools;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
      n: usize,
      a: [usize; n],
      k: usize,
    }

    let mut count = 0;
    for i in 0..n {
        if a[i] >= k {
            count += 1;
        }
    }
    println!("{count}");
}
