use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
     s: Chars,
    }

    let res = ['A', 'B', 'C'].iter().all(|&c| s.contains(&c));
    println!("{}", if res { "Yes" } else { "No" });
}
