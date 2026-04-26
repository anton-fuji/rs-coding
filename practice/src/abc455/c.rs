use proconio::{fastout, input, marker::Chars};
use std::collections::HashMap;

#[fastout]
#[allow(unused_imports, non_snake_case)]
fn main() {
    input! {
        n: usize,
        k: usize,
        a :[i64; n]
    }

    let mut m: HashMap<i64, i64> = HashMap::new();
    for &a in &a {
        *m.entry(a).or_insert(0) += a;
    }

    let mut sum: Vec<i64> = m.values().cloned().collect();
    sum.sort_unstable_by(|a, b| b.cmp(a));

    let total: i64 = a.iter().sum();
    let remove: i64 = sum.iter().take(k).sum();
    println!("{}", total - remove);
}
