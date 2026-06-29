use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
        s: Chars,
    }

    let mut e = 0;
    let mut w = 0;

    for c in s {
        if c == 'E' {
            e += 1;
        } else {
            w += 1;
        }
    }
    println!("{}", if e > w { "East" } else { "West" });
}
