use std::collections::HashSet;

use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
      s: [String; n],
    }

    let mut set = HashSet::new();

    for i in 0..n {
        for j in 0..n {
            if s[i] != s[j] {
                let res = format!("{}{}", s[i], s[j]);
                set.insert(res);
            }
        }
    }
    println!("{}", set.len());
}
