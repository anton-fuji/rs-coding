use itertools::Itertools;
use proconio::{fastout, input, marker::Usize1};
use std::usize::MAX;

#[fastout]
fn main() {
    input! {
      n : usize,
      a: [[usize; n];n],
      m: usize,
      xy: [(Usize1, Usize1); m]
    }

    let mut ng = vec![vec![false; n]; n];
    for (x, y) in xy {
        ng[x][y] = true;
        ng[y][x] = true;
    }

    let mut res = MAX;
    for pat in (0..n).permutations(n) {
        let mut ns = 0;
        let mut ok = true;
        for i in 0..n {
            ns += a[pat[i]][i];
            if i != n - 1 && ng[pat[i]][pat[i + 1]] {
                ok = false;
                break;
            }
        }
        if ok {
            res = res.min(ns);
        }
    }
    if res == MAX {
        println!("-1");
    } else {
        println!("{}", res);
    }
}
