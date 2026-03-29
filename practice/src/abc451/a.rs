use proconio::{fastout, input, marker::Chars};

#[fastout]
#[allow(unused_imports)]
fn main() {
    input! {
        s: Chars,
    }

    let res = s.len() % 5 == 0;
    println!("{}", if res { "Yes" } else { "No" });
}
