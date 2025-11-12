use itertools::Itertools;
use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
      s :Chars,
    }

    let mut x = s
        .iter()
        .map(|&c| c.to_digit(10).unwrap() as i32)
        .collect_vec();
    x.push(0);
    let y = (0..s.len())
        .map(|i| ((x[i] - x[i + 1]) % 10 + 10) % 10)
        .collect_vec();
    println!("{}", sum(y));
}

fn sum(v: Vec<i32>) -> i32 {
    let mut ret = v.iter().sum();
    ret += v.len() as i32;
    ret
}
