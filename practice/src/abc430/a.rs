use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
      a: usize,
      b: usize,
      c: usize,
      d: usize,
    }

    let mut res = "Yes";
    if a <= c {
        if b <= d {
            res = "No";
        }
    } else if a > c {
        res = "No";
    }

    println!("{res}");
}
