use itertools::*;
use proconio::{marker::*, *};

#[fastout]
fn main() {
    input! {
        n:usize,
      q: usize,
     mut a: [i64; n],
     mut b: [i64; n]
    }

    let mut r = izip!(&a, &b).map(|(a_i, b_i)| a_i.min(b_i)).sum::<i64>();
    for _ in 0..q {
        input! {c: char, x: Usize1, v: i64}
        r -= a[x].min(b[x]);
        match c {
            'A' => a[x] = v,
            'B' => b[x] = v,
            _ => {}
        }
        r += a[x].min(b[x]);
        println!("{r}");
    }
}
