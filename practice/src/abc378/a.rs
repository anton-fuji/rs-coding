use std::collections::HashMap;

use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
       a: [usize; 4]
    }

    let mut map: HashMap<usize, usize> = HashMap::new();
    for v in a {
        *map.entry(v).or_insert(0) += 1;
    }

    let mut count = 0;
    for (_, v) in map {
        if v == 4 {
            count += 2;
        } else if v == 3 || v == 2 {
            count += 1;
        } else {
        }
    }
    println!("{count}");
}
