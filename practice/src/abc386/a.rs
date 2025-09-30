use std::collections::HashSet;

use proconio::input;

fn main() {
    input! {
     a: [usize; 4]
    }

    let set: HashSet<usize> = HashSet::from_iter(a);
    let res = set.len() == 2;

    println!("{}", if res { "Yes" } else { "No" });
}
