use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
      n: usize,
      cp: [(usize, usize); n],
      q :usize,
      lr: [(usize, usize); q]
    }

    let mut sum1 = vec![0; n + 1];
    let mut sum2 = vec![0; n + 1];
    for i in 0..n {
        let (c, p) = cp[i];
        sum1[i + 1] = sum1[i];
        sum2[i + 1] = sum2[i];
        if c == 1 {
            sum1[i + 1] += p;
        } else {
            sum2[i + 1] += p;
        }
    }

    for &(l, r) in &lr {
        let s1 = sum1[r] - sum1[l - 1];
        let s2 = sum2[r] - sum2[l - 1];
        println!("{s1} {s2}");
    }
}
