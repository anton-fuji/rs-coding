use proconio::marker::Chars;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: Chars,
    }

    let a = n[0];
    let b = n[1];
    let c = n[2];

    print!("{b}{c}{a}");
    print!(" ");
    print!("{c}{a}{b}\n");
}
