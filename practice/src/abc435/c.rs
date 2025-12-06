use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
      n : usize,
      a: [usize; n]
    }

    let mut reach = a[0];
    let mut i = 0;

    while i < reach.min(n) {
        let extend = i + a[i];
        reach = reach.max(extend);
        i += 1;
    }
    println!("{}", reach.min(n));
}
