use proconio::{fastout, input};
use std::collections::BTreeMap;

#[fastout]
fn main() {
    input! {
        _n: usize,
        k: usize,
        s: String,
    }
    let mut m: BTreeMap<String, usize> = BTreeMap::new();
    for c in s.chars().collect::<Vec<_>>().windows(k) {
        let str = c.iter().collect::<String>();
        let count = m.entry(str).or_insert(0);
        *count += 1;
    }

    let x = *m.values().max().unwrap();

    println!("{}", x);
    for (key, value) in m {
        if value == x {
            print!("{} ", key);
        }
    }
}
