use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
      x: i64,
      y: i64,
    }

    let mut count: f64 = 0.0;
    for i in 1..=6 {
        for j in 1..=6 {
            if i + j >= x || (i - j).abs() >= y {
                count += 1.0
            }
        }
    }

    let res: f64 = count / 36.0;
    println!("{:.10}", res);
}
