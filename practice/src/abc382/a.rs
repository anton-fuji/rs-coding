use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
      n: usize,
      d: usize,
      s:Chars
    }

    let mut empty = 0;
    let mut at = 0;
    for c in &s {
        if *c == '.' {
            empty += 1;
        } else {
            at += 1;
        }
    }

    let mut res = 0;
    if at >= d {
        res = empty + d;
    } else {
        println!("{empty}");
    }
    println!("{res}");
}
