use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
      a: usize,
    }

    let b = 400 / a;
    if (400 % a) == 0 {
        println!("{b}");
    } else {
        println!("-1");
    }
}
