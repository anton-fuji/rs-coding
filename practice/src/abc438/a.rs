use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
      d: usize,
      f: usize,
    }

    let res = (d - f) % 7;
    println!("{}", 7 - res)
}
