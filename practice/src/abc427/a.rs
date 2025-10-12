use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
       s: Chars,
    }

    let l = s.len();
    let x: String = s
        .iter()
        .enumerate()
        .filter(|(i, _c)| *i != l / 2)
        .map(|(_, c)| c)
        .collect();

    println!("{x}");
}
