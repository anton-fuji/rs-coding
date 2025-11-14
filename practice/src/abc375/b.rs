use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
      n: isize,
    }

    let mut nx = 0isize;
    let mut ny = 0isize;
    let mut sum = 0f64;
    for _ in 0..n {
        input! {x:isize, y:isize}
        let dx = (nx - x) as f64;
        let dy = (ny - y) as f64;
        sum += (dx * dx + dy * dy).sqrt();
        nx = x;
        ny = y;
    }

    let dx = nx as f64;
    let dy = ny as f64;
    sum += (dx * dx + dy * dy).sqrt();
    println!("{:.10}", sum);
}
