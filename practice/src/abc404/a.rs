use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
      s: String,
    }

    for val in 'a'..='z' {
        if !s.contains(val) {
            println!("{val}");
            return;
        }
    }
}
