use itertools::Itertools;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
      n: usize,
      m: usize,
    }

    let mut g = vec![Vec::new(); n];
    for _ in 0..m {
        input! {a: usize, b:usize}

        let a = a - 1;
        let b = b - 1;

        g[a].push(b);
        g[b].push(a);
    }

    let mut res = vec![0; n];
    for i in 0..n {
        let k = n as isize - g[i].len() as isize - 1;
        res[i] = k * (k - 1) * (k - 2) / 6;
    }
    println!("{}", res.iter().join(" "));
}
