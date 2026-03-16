use proconio::{fastout, input, marker::Chars};
use std::f64::consts::PI;

#[fastout]
#[allow(unused_imports)]
fn main() {
    input! {
        d: f64,
    }
    let r = d / 2.0;
    println!("{}", r * r * PI);
}
