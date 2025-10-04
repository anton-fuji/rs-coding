use itertools::Itertools;
use proconio::{fastout, input};
use std::collections::HashMap;

#[fastout]
fn main() {
    input! {
      s: String
    }

    let map: HashMap<_, _> = s.chars().counts();
    for (k, v) in &map {
        if *v == 1 {
            println!("{k}");
            return;
        }
    }
}
