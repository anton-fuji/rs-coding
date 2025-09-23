use std::collections::HashSet;

use itertools::Itertools;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
      n: u32,
      m: usize,
      a: [u32; m]
    }

    let set: HashSet<_> = a.into_iter().collect();
    let res: Vec<_> = (1..=n).filter(|x| !set.contains(x)).collect();

    println!("{}", res.len());
    println!("{}", res.iter().join(" "));
}
