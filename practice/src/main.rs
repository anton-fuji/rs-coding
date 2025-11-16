use itertools::Itertools;
use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
      v: Chars,
    }

    let res = v
        .clone()
        .into_iter()
        .permutations(v.len())
        .filter(|f| f[0] != '0')
        .map(|p| p.iter().collect::<String>())
        .max()
        .unwrap();
    println!("{res}");
}
