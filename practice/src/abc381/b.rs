use std::collections::HashMap;

use proconio::marker::Chars;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        s: Chars,
    }

    let mut ok = true;
    let mut map: HashMap<_, _> = HashMap::new();
    let n = s.len();
    if n % 2 != 0 {
        ok = false;
    }

    for j in 0..n {
        *map.entry(s[j]).or_insert(0) += 1;
    }

    for (_, &cnt) in map.iter() {
        if cnt > 2 {
            ok = false;
            break;
        }
    }

    for i in 1..=(n / 2) {
        let tmp = (i * 2) - 1;
        if s[tmp - 1] != s[tmp] {
            ok = false;
            break;
        }
    }
    println!("{}", if ok { "Yes" } else { "No" });
}
