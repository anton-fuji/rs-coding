use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
      a: usize,
      b: usize,
      c: usize,
    }

    if a == b && b == c {
        println!("Yes");
    } else if a == c {
        println!("Yes");
    } else if b == c {
        println!("Yes");
    } else if a == b {
        println!("Yes");
    } else {
        println!("No");
    }
}
