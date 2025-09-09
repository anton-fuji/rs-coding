// use std::{cmp::Reverse, collections::BinaryHeap};

// use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
      n: usize,
      m: usize,
      a: [usize; n],
    }

    let mut sum = 0;
    for i in 0..n {
        sum += a[i];
    }
    if sum <= m {
        println!("Yes");
    } else {
        println!("No");
    }
}
