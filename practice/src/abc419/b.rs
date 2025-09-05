use proconio::input;
use std::cmp::Reverse;
use std::collections::BinaryHeap;

fn main() {
    input! {
        q_size: usize,
    }

    let mut heap = BinaryHeap::new();
    for _ in 0..q_size {
        input! {
          t:usize
        };

        if t == 1 {
            input! {x:i32}
            heap.push(Reverse(x));
        } else {
            println!("{}", heap.pop().unwrap().0);
        }
    }
}
