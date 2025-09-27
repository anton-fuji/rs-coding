use itertools::*;
use proconio::{marker::*, *};

#[fastout]
fn main() {
    input! {
        n: usize, m: usize,
        s: [Bytes; n]
    }
    let mut r = vec![0; n];
    for j in 0..m {
        let mut zero = vec![];
        let mut one = vec![];
        for i in 0..n {
            if s[i][j] == b'0' {
                zero.push(i)
            } else {
                one.push(i)
            }
        }
        if zero.len() > one.len() {
            for i in one {
                r[i] += 1
            }
        } else {
            for i in zero {
                r[i] += 1
            }
        }
    }
    let r_max = *r.iter().max().unwrap();
    println!("{}", (1..=n).filter(|&i| r[i - 1] >= r_max).join(" "));
}
