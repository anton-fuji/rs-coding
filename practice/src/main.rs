use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
    }
    for _ in 0..n {
        input! {
            x1: i64,
            y1: i64,
            r1: i64,
            x2: i64,
            y2: i64,
            r2: i64,
        }
        let d = (y2 - y1) * (y2 - y1) + (x2 - x1) * (x2 - x1);

        if d < (r2 - r1) * (r2 - r1) || d > (r1 + r2) * (r1 + r2) {
            println!("No");
        } else {
            println!("Yes");
        }
    }
}
