use itertools::Itertools;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
      n: usize,
      p: [usize; n],
      q: [usize; n],
    }

    let mut p2p = vec![0; n + 1];
    let mut b2p = vec![0; n + 1];
    let mut p2b = vec![0; n + 1];

    for i in 0..n {
        p2p[i + 1] += p[i];
        p2b[i + 1] += q[i];
        b2p[q[i]] += i + 1;
    }

    let mut res = vec![];
    for i in 1..=n {
        let p_from = b2p[i];
        let p_to = p2p[p_from];
        let bp2 = p2b[p_to];
        res.push(bp2);
    }
    println!("{}", res.iter().join(" "));
}
