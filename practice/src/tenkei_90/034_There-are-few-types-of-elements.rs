use std::cmp::max;
use std::collections::HashMap;

use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
      n : usize,
      k : usize,
      a: [i64; n]
    }

    // しゃくとり法を使う
    let mut cnts = HashMap::new();
    let mut left = 0;
    let mut max_len = 0;

    for right in 0..n {
        let r_val = a[right];
        *cnts.entry(r_val).or_insert(0) += 1;

        // leftがkを超過した時、leftを0になるまでデクリメント
        while cnts.len() > k {
            let l_val = a[left];
            if let Some(cnt) = cnts.get_mut(&l_val) {
                *cnt -= 1;
                if *cnt == 0 {
                    cnts.remove(&l_val);
                }
            }
            left += 1;
        }
        max_len = max(max_len, right - left + 1);
    }
    println!("{}", max_len);
}
