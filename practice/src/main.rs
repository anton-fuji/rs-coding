use std::collections::HashSet;

use proconio::{fastout, input, marker::Chars};

#[fastout]
#[allow(unused_imports)]
fn main() {
    input! {
        mut n: u64,
    }

    let mut seen = HashSet::new();

    while n != 1 && !seen.contains(&n) {
        seen.insert(n);
        n = suquare_sum(n);
    }

    if n == 1 {
        println!("Yes");
    } else {
        println!("No");
    }
}

fn suquare_sum(mut n: u64) -> u64 {
    let mut sum = 0;

    while n > 0 {
        let d = n % 10;
        sum += d * d;
        n /= 10;
    }
    sum
}
