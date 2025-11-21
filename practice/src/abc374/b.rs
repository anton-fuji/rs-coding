use proconio::{fastout, input, marker::Chars};
use std::cmp::min;

#[fastout]
fn main() {
    input! {
      s: Chars,
      t: Chars,
    }

    if s == t {
        println!("0");
        return;
    }

    let l = min(s.len(), t.len());
    let mut res = l + 1;

    for i in 0..l {
        if s[i] != t[i] {
            res = i + 1;
            println!("{res}");
            return;
        }
    }
    println!("{res}");
}
