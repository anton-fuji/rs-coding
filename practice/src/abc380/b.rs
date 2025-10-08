use proconio::marker::Chars;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        s: Chars,
    }

    let n = s.len();
    let mut res = Vec::new();
    let mut cnt = 0;
    for i in 1..n {
        match s[i] {
            '-' => {
                cnt += 1;
            }
            _ => {
                res.push(cnt.to_string());
                cnt = 0;
            }
        }
    }
    println!("{}", res.join(" "))
}
