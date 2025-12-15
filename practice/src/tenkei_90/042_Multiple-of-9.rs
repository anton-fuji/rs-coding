use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
      k: usize,
    }

    if k % 9 != 0 {
        println!("0");
        return;
    }

    let modu = 1_000_000_007;
    let mut dp = vec![0; k + 1];
    dp[0] = 1;
    for i in 1..=k {
        for d in 1..=9 {
            if i < d {
                break;
            }
            dp[i] += dp[i - d];
            dp[i] %= modu;
        }
    }

    println!("{}", dp[k]);
}
