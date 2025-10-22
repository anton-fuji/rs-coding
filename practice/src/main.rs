use std::collections::HashSet;

use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
       card: [usize; 4]
    }

    let set: HashSet<usize> = HashSet::from_iter(card);
    let res = set.len() == 2;
    println!("{}", if res { "Yes" } else { "No" });
}
