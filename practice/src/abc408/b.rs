use proconio::{fastout, input};
use std::collections::HashSet;

#[fastout]
fn main() {
    input! {
      n: usize,
      a: [i64; n]
    }

    let ans = solve(&a);
    let cnt = ans.iter().count();
    println!("{cnt}");
    for (i, v) in ans.iter().enumerate() {
        if i > 0 {
            print!(" ");
        }
        print!("{v}");
    }
    println!();
}

fn solve(a: &[i64]) -> Vec<i64> {
    let mut set: HashSet<i64> = HashSet::new();

    for &a in a {
        set.insert(a);
    }
    let mut res: Vec<i64> = set.into_iter().collect();
    res.sort();
    res
}
