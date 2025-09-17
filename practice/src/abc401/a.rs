use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
      s: usize,
    }

    if s >= 200 && s <= 299 {
        println!("Success");
    } else {
        println!("Failure");
    }
}
