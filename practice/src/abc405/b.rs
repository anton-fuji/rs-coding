use std::collections::HashSet;

use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
      n: usize,
      m: usize,
    mut  a: [usize; n],
    }

    let mut count = 0;
    while a.iter().collect::<HashSet<_>>().len() == m {
        count += 1;
        a.pop();
    }

    println!("{count}");
}
