use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
      n: u64,
      m: u64,
    }

    let mut sum: u128 = 0;
    let mut power: u128 = 1;
    for _ in 0..=m {
        sum += power;

        if sum > 10u128.pow(9) {
            println!("inf");
            return;
        }
        power *= n as u128;
    }
    println!("{sum}");
}
