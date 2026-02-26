use proconio::{fastout, input, marker::Chars};

#[fastout]
#[allow(unused_imports)]
fn main() {
    input! {
        s: Chars
    }
    let l = s.len() - 1;
    println!("{}", if s[0] == s[l] { "Yes" } else { "No" });
}
