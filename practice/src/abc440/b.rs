use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
      n: usize,
      t: [usize; n],
    }
    let mut v: Vec<(usize, usize)> = t.iter().cloned().enumerate().collect();
    v.sort_by_key(|&(_, a)| a);
    for (i, _) in v.iter().take(3) {
        println!("{}", i + 1);
    }
}
