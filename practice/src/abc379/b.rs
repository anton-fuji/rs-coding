use proconio::marker::Chars;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
       _n : usize,
        k: usize,
        s: Chars,
    }

    let res = s
        .split(|&c| c == 'X')
        .map(|seg| seg.len() / k)
        .sum::<usize>();

    println!("{res}");
}
