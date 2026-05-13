use proconio::{fastout, input, marker::Chars};

#[fastout]
#[allow(unused_imports, non_snake_case)]
fn main() {
    input! {
        n: usize,
        l: usize,
        r: usize,
        s: Chars,
    }

    let mut cnt = vec![vec![0i64; n + 1]; 26];
    for i in 0..n {
        let c = (s[i] as u8 - b'a') as usize;
        for k in 0..26 {
            cnt[k][i + 1] = cnt[k][i];
        }
        cnt[c][i + 1] += 1;
    }

    let mut res = 0;
    for j in 0..n {
        let c = (s[j] as u8 - b'a') as usize;
        let hi = if j >= l { j - l } else { continue };
        let lo = if j >= r { j - r } else { 0 };

        res += cnt[c][hi + 1] - cnt[c][lo];
    }
    println!("{}", res);
}
