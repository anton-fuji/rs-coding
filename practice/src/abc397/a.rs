use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
      x: f64,
    }

    if x >= 38.0 {
        println!("1");
    } else if x >= 37.5 && x < 38.0 {
        println!("2");
    } else {
        println!("3");
    }
}
