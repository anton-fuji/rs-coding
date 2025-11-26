use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
      (a, b, c): (usize, usize,usize),
    }

    let cc = c.pow(b as u32);
    if a < cc {
        println!("Yes");
    } else {
        println!("No");
    }
}
