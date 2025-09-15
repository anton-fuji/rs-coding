use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
      a: usize,
      b:usize,
    }

    let res = (2 * a + b) / (2 * b);
    println!("{res}");
}
