use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
      h: usize,
      b: usize,
    }

    if h > b {
        if (h - b) > 0 {
            println!("{}", h - b);
        } else {
            println!("0");
        }
    } else {
        println!("0");
    }
}
