use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
      x: i64,
      y: i64,
    }

    let mut cnt = 0;

    for i in 1..=6 {
        for j in 1..=6 {
            let sum = i + j;
            let abs = ((i - j) as f64).abs();
            if x <= sum || y as f64 <= abs {
                cnt += 1;
            }
        }
    }
    println!("{}", cnt as f64 / 36.0);
}
