use std::collections::HashMap;

use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
      n: usize,
      a: [usize; n]
    }

    let mut m: HashMap<usize, usize> = HashMap::new();
    for &a in &a {
        *m.entry(a).or_insert(0) += 1;
    }

    let mut res = 0;
    for (_key, value) in m.iter() {
        if *value >= 2 {
            res += c2(*value) * (n - *value);
        }
    }
    println!("{res}");
}

fn c2(c: usize) -> usize {
    return c * (c - 1) / 2;
}
