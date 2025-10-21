use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
      n: usize,
      s: [String; n],
      x: usize,
      y: String,
    }

    let mut res = "No";
    for (key, val) in s.iter().enumerate() {
        if key == (x - 1) && *val == y {
            res = "Yes";
        }
    }
    println!("{res}");
}
