// use std::{cmp::Reverse, collections::BinaryHeap};
// use itertools::Itertools;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
      n: usize,
      t: String,
      a: String,
    }

    let t_list: Vec<char> = t.chars().collect();
    let a_list: Vec<char> = a.chars().collect();
    for i in 0..n {
        if t_list[i] == 'o' && a_list[i] == 'o' {
            println!("Yes");
            return;
        }
    }
    println!("No");
}
