use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
      mut s: Chars,
    }

    s.sort_unstable();
    let res = if s[0] == s[1] { s[s.len() - 1] } else { s[0] };
    println!("{res}");
}
