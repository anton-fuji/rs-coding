use std::collections::BTreeMap;

use proconio::{fastout, input, marker::Chars};

#[fastout]
#[allow(unused_imports, non_snake_case)]
fn main() {
    input! {
        n:usize,
        q: usize
    }

    let mut blocks = vec![0i64; n + 1];
    let mut offset = 0i64;
    let mut cnt = BTreeMap::new();
    cnt.insert(0i64, n);

    for _ in 0..q {
        input! {t: usize, v: i64}
        if t == 1 {
            let x = v as usize;
            *cnt.entry(blocks[x]).or_insert(0) -= 1;
            if cnt[&blocks[x]] == 0 {
                cnt.remove(&blocks[x]);
            }
            blocks[x] += 1;
            *cnt.entry(blocks[x]).or_insert(0) += 1;

            let min_val = *cnt.keys().next().unwrap();
            if min_val > offset {
                offset += 1;
            }
        } else {
            let y = offset + v;
            let count: usize = cnt.range(y..).map(|(_, &f)| f).sum();
            println!("{count}");
        }
    }
}
