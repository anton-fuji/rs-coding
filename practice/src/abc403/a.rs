use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
      n: usize,
     a: [usize; n],
    }

    let mut sum = 0;
    for (_, i) in (0..n).enumerate() {
        if i % 2 == 0 {
            sum += a[i];
        }
    }
    println!("{sum}");
}
