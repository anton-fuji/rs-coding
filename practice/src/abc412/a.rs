// use std::{cmp::Reverse, collections::BinaryHeap};
// use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
      n: usize,
      ab: [(i64, i64); n],
    }

    let res = ab.iter().filter(|(a, b)| a < b).count();
    println!("{res}");
}
