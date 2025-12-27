use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
      t: usize,
    }

    let res = fx(fx(fx(t) + t) + fx(fx(t)));
    println!("{res}");
}

fn fx(x: usize) -> usize {
    x.pow(2) + 2 * x + 3
}
