// use std::{cmp::Reverse, collections::BinaryHeap};
use std::collections::HashSet;
// use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
      n: usize,
      s: [String; n],
    }

    let mut dict = HashSet::new();
    for i in 0..n {
        for j in 0..n {
            if i != j {
                let combind_str = format!("{}{}", s[i], s[j]);
                dict.insert(combind_str);
            }
        }
    }
    println!("{}\n", dict.len());
}
