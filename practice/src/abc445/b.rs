use std::iter::repeat;

use proconio::{fastout, input, marker::Chars};

#[fastout]
#[allow(unused_imports)]
fn main() {
    input! {
        n: usize,
        s: [String; n],
    }

    let max_len = s.iter().map(|w| w.len()).max().unwrap();
    for c in s {
        let sub = max_len - c.len();
        let d = sub / 2;
        let l_d: String = repeat('.').take(d).collect();
        let r_d: String = repeat('.').take(d).collect();

        let res = format!("{}{}{}", l_d, c, r_d);
        println!("{}", res);
    }
}
