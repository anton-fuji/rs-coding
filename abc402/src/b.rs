use proconio::input;
use std::collections::VecDeque;

fn main() {
    input! {
      q:usize ,
    }

    // FIFOに利用
    let mut queries = VecDeque::new();
    for _ in 0..q {
        input! {
          t: usize,
        }

        if t == 1 {
            input! {x : usize,}
            queries.push_back(x);
        } else {
            let ans = queries.pop_front().unwrap();
            println!("{}", ans)
        }
    }
}
