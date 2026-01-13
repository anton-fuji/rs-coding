use itertools::Itertools;
use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
      n: usize,
      d: usize,
      mut s: Chars,
    }

    s.reverse();
    let mut cnt = 0;
    for i in 0..n {
        if cnt == d {
            break;
        }
        if s[i] == '@' {
            s[i] = '.';
            cnt += 1;
        }
    }
    s.reverse();
    println!("{}", s.iter().join(""));
}
