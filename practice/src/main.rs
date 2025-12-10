use proconio::{fastout, input};
const MAX: isize = 1_000_000_000_000_000;

#[fastout]
fn main() {
    input! {
      n: usize,
      mut a: [isize; n],
      q: usize,
    }
    a.push(MAX);
    a.push(-MAX);

    a.sort();
    for _ in 0..q {
        input! {b: isize}
        let idx = match a.binary_search(&b) {
            Ok(i) | Err(i) => i,
        };
        let res = (a[idx] - b).abs().min((a[idx - 1] - b).abs());
        println!("{res}");
    }
}
