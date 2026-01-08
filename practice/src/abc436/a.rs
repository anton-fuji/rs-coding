use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
      n: usize,
      s: String
    }

    let sub = n - s.len();
    let res = "o".repeat(sub) + &s;
    println!("{res}");
}
