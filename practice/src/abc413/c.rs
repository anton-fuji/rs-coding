use std::collections::VecDeque;

use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
    }

    let mut v = VecDeque::new();
    for _ in 0..n {
        input! {
          q_type: usize,
        }
        match q_type {
            1 => {
                input! {c:usize, x: usize}
                v.push_back((c, x));
            }
            2 => {
                input! { mut k: usize};
                let mut res = 0;
                while k > 0 {
                    let (c, x) = v.pop_front().unwrap_or((0, 0));
                    if c > k {
                        v.push_front((c - k, x));
                        res += x * k;
                        k = 0;
                    } else {
                        res += c * x;
                        k -= c;
                    }
                }
                println!("{res}");
            }
            _ => unreachable!(),
        }
    }
}
