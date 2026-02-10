use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
      p: usize,
      q: usize,
      x: usize,
      y: usize,
    }

    let pi = p + 99;
    let qi = q + 99;
    if p <= x && q <= y {
        if x <= pi && y <= qi {
            println!("Yes");
        } else {
            println!("No");
        }
    } else {
        println!("No");
    }
}
