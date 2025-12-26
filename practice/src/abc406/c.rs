use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
      n: usize,
      p: [i64; n],
    }

    let mut sign = Vec::with_capacity(n - 1);
    for i in 0..n - 1 {
        if p[i] < p[i + 1] {
            sign.push(1);
        } else {
            sign.push(-1);
        }
    }

    let mut runs: Vec<(i32, i64)> = Vec::new();
    for &s in &sign {
        if let Some((last_s, last_len)) = runs.last_mut() {
            if *last_s == s {
                *last_len += 1;
            } else {
                runs.push((s, 1));
            }
        } else {
            runs.push((s, 1));
        }
    }
    let mut res = 0;
    for i in 0..runs.len().saturating_sub(2) {
        let (s1, a) = runs[i];
        let (s3, c) = runs[i + 2];

        if s1 == 1 && s3 == 1 {
            res += a * c
        }
    }

    println!("{res}");
}
