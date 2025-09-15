// use std::{cmp::reverse, collections::binaryheap};
// use itertools::itertools;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
      n: usize,
      s: f64,
      t: [f64; n]
    }

    let time = s + 0.5;
    let mut tyourou = "Yes";
    let mut later: f64;

    for i in 0..n {
        if i == 0 {
            later = t[i];
        } else {
            later = t[i] - t[i - 1];
        }
        if later >= time {
            tyourou = "No";
            break;
        }
    }
    println!("{tyourou}");
}
