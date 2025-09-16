use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
      n: usize,
      k: u32,
      a: [u128; n],
    }

    let mut res = 1;
    let lim = 10_u128.pow(k);
    for i in 0..n {
        res *= a[i];
        if res >= lim {
            res = 1;
        }
    }
    println!("{res}");
}
