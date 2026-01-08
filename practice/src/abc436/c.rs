use std::collections::HashSet;

use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        _n: usize,
      m: usize
    }

    let mut set: HashSet<(usize, usize)> = HashSet::new();
    let mut res = 0;
    for _ in 0..m {
        input! {r:usize, c:usize}
        let cells = [(r, c), (r + 1, c), (r, c + 1), (r + 1, c + 1)];

        if cells.iter().any(|&v| set.contains(&v)) {
            continue;
        }

        for p in cells {
            set.insert(p);
        }
        res += 1;
    }

    println!("{res}");
}
