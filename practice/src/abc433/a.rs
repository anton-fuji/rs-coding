use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
      mut x: usize,
      mut y: usize,
      z: usize,
    }

    let mut res = "No";
    while y <= 100 {
        if x == y * z {
            res = "Yes";
            break;
        } else {
            x += 1;
            y += 1;
        }
    }

    println!("{res}");
}
