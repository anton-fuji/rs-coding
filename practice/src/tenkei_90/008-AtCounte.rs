use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
      n: usize,
      s: Chars,
    }

    let target = "atcoder".chars().collect::<Vec<char>>();
    let mod_num = 1_000_000_007;
    let mut dp = vec![vec![0; target.len() + 1]; n + 1];

    for i in 0..=n {
        dp[i][0] = 1;
    }
    for i in 1..=n {
        for j in 1..=target.len() {
            dp[i][j] += dp[i - 1][j];
            if s[i - 1] == target[j - 1] {
                dp[i][j] += dp[i - 1][j - 1];
            }
            dp[i][j] %= mod_num;
        }
    }

    println!("{}", dp[n][target.len()]);
}
