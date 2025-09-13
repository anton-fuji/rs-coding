// use std::{cmp::Reverse, collections::BinaryHeap};
// use itertools::Itertools;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
      p: String,
      l: usize,
    }

    if p.len() >= l {
        println!("Yes");
    } else {
        println!("No");
    }
}
