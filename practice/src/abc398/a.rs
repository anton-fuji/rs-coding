use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
      n: usize,
    }

    if n % 2 == 0 {
        for i in 0..n {
            if i == (n - 1) / 2 || i == n / 2 {
                print!("=");
            } else {
                print!("-");
            }
        }
    } else {
        for i in 0..n {
            if i == n / 2 {
                print!("=");
            } else {
                print!("-");
            }
        }
    }
    println!();
}
