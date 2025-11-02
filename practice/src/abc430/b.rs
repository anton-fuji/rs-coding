use std::collections::HashSet;

use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
      n: usize,
      m: usize,
      grid: [String; n]
    }

    let mut set = HashSet::new();
    for i in 0..=(n - m) {
        for j in 0..=(n - m) {
            let mut pattern = String::new();

            for di in 0..m {
                pattern.push_str(&grid[i + di][j..j + m]);
            }
            set.insert(pattern);
        }
    }
    println!("{}", set.len());
}
