use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
      (a, b, c): (usize, usize,usize),
    }

    let r = gcd(a, gcd(b, c));
    let res = (a / r - 1) + (b / r - 1) + (c / r - 1);
    println!("{res}");
}

fn gcd(mut a: usize, mut b: usize) -> usize {
    while b != 0 {
        (a, b) = (b, a % b);
    }
    a
}
