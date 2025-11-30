use std::collections::HashMap;

use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
      n: usize,
    }

    let mut reg: HashMap<String, u64> = HashMap::new();

    for i in 0..n {
        input! {s :String}
        let cnt = reg.entry(s).or_insert(0);
        if *cnt == 0 {
            println!("{}", i + 1);
        }
        *cnt += 1;
    }
}
