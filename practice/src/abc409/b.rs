// use std::{cmp::Reverse, collections::BinaryHeap};
// use itertools::Itertools;
use proconio::{fastout, input};
use std::cmp::Reverse;

#[fastout]
fn main() {
    input! {
      n: usize,
      mut a: [usize; n],
    }

    a.sort_unstable_by_key(|&x| Reverse(x));
    let a: Vec<_> = a.into_iter().enumerate().collect();
    let res = a.partition_point(|&(i, x)| x > i);

    println!("{res}");
}
