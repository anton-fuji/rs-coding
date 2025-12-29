use std::collections::HashSet;

use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
      _n: usize,
      m: usize,
      edges : [(usize, usize); m],
    }

    let mut set = HashSet::new();
    let mut remove = 0;

    for (u, v) in edges {
        if u == v {
            remove += 1;
        } else {
            let a = u.min(v);
            let b = u.max(v);
            if !set.insert((a, b)) {
                remove += 1;
            }
        }
    }
    println!("{remove}");
}
