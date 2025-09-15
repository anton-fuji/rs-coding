use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
      a: i64,
      b: i64,
      c: i64,
      d: i64,
    }

    if c < a || (c == a && d <= b) {
        println!("Yes");
    } else {
        println!("No");
    }
}
