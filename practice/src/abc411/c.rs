// use std::{cmp::Reverse, collections::BinaryHeap};
// use itertools::Itertools;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
      n: usize,
      q: usize,
      a: [usize; q],
    }

    let mut flg = vec![0; n + 1];
    let mut res = 0;

    for a in a {
        let before = flg[a - 1] + flg[a];
        flg[a - 1] ^= 1;
        flg[a] ^= 1;
        let after = flg[a - 1] + flg[a];
        res += (after - before) / 2;
        println!("{res}");
    }
}
