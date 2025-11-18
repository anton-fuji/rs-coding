use itertools::Itertools;
use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
      x: Chars,
    }

    let res = x
        .clone()
        .into_iter()
        .permutations(x.len())
        .filter(|p| p[0] != '0')
        .map(|x| x.iter().collect::<String>())
        .min()
        .unwrap();

    println!("{res}");
}
