use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
       n:usize,
       m:usize,
    }

    let mut v = vec![0; m + 1];
    let mut cnt = vec![0; m + 1];
    for _ in 0..n {
        input! {
          a: usize,
          b:usize,
        }

        cnt[a] += 1;
        v[a] += b;
    }

    for i in 1..m + 1 {
        let r = v[i] as f64 / cnt[i] as f64;
        println!("{:.10}", r);
    }
}
