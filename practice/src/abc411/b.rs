// use std::{cmp::Reverse, collections::BinaryHeap};
// use itertools::Itertools;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
      n: usize,
      d: [i64; n-1],
    }

    for i in 0..n - 1 {
        let mut tmp = 0;
        for j in i..n - 1 {
            tmp += d[j];
            print!("{tmp}");
            if j != n - 2 {
                print!(" ");
            }
        }
        println!();
    }
}
