use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
      n: usize,
      l: usize,
      d: [usize; n-1]
    }

    if l % 3 != 0 {
        println!("0");
        return;
    }

    let mut count = vec![0usize; l];
    let mut idx = 0;
    count[idx] += 1;
    for &d in &d {
        idx += d;
        idx %= l;
        count[idx] += 1;
    }

    let mut total = 0;
    let d = l / 3;
    for i in 0..d {
        total += count[i] * count[i + d] * count[i + 2 * d];
    }

    println!("{total}");
}
