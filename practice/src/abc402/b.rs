use std::collections::VecDeque;

use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
      q: usize,
    }

    let mut low = VecDeque::new();
    for _ in 0..q {
        input! {
          q_type: i64,
        }
        match q_type {
            1 => {
                input! {
                  x : usize,
                }
                low.push_back(x);
            }
            2 => {
                if let Some(l) = low.pop_front() {
                    println!("{}", l);
                }
            }
            _ => {
                return;
            }
        }
    }
}
