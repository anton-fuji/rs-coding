use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
      s: String,
    }

    let res: String = s.chars().filter(|c| ('A'..='Z').contains(c)).collect();

    println!("{res}");
}
