use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
      n: usize,
      s: Chars
    }

    let mut pos_a = Vec::new();
    for (i, &c) in s.iter().enumerate() {
        if c == 'A' {
            pos_a.push(i as i64);
        }
    }

    let mut cost1 = 0i64;
    let mut cost2 = 0i64;

    for i in 0..n {
        cost1 += (pos_a[i] - (2 * i) as i64).abs();
        cost2 += (pos_a[i] - (2 * i + 1) as i64).abs();
    }

    println!("{}", cost1.min(cost2));
}
