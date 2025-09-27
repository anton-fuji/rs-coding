use itertools::*;
use proconio::{marker::*, *};

#[fastout]
fn main() {
    input! {
        x:usize,
      y: usize,
    }

    let res = x + y;
    if res > 12 {
        println!("{}", res - 12);
    } else {
        println!("{res}");
    }
}
