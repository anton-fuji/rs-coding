use proconio::{fastout, input};

const MOD: i64 = 10i64.pow(9);

#[fastout]
fn main() {
    input! {
      n: usize,
      k: usize,
    }

    let mut a = vec![0i64; n + 1];
    let mut s = vec![0i64; n + 2];

    for i in 0..k.min(n + 1) {
        a[i] = 1;
        s[i + 1] = (s[i] + a[i]) % MOD;
    }

    for i in k..=n {
        a[i] = (s[i] - s[i - k] + MOD) % MOD;
        s[i + 1] = (s[i] + a[i]) % MOD;
    }

    println!("{}", a[n]);
}
