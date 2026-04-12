use proconio::{fastout, input, marker::Chars};

#[fastout]
#[allow(unused_imports)]
fn main() {
    input! {
        n: usize,
        s: Chars
    }
    let idx = (0..n).find(|&i| s[i] != 'o').unwrap_or(n);
    let res: String = s[idx..].iter().collect();
    println!("{}", res);
}
