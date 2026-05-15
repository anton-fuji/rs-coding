use proconio::{fastout, input, marker::Chars};

#[fastout]
#[allow(unused_imports, non_snake_case)]
fn main() {
    input! {
        n: usize,
        k: usize,
        x: i64,
        mut a: [i64; n]
    }

    // 大きい順にソート
    a.sort_unstable_by(|a, b| b.cmp(a));

    // 累積和
    let mut prefix = vec![0i64; n + 1];
    for i in 0..n {
        prefix[i + 1] = prefix[i] + a[i];
    }

    // m個選んだとき確実にX ml以上飲めるか判定
    let check = |m: usize| -> bool {
        // 選ばなかったN-m個に日本酒を詰め込める最大数
        // → 選んだm個に確実に含まれる日本酒の数
        let sake_min = if k > n - m { k - (n - m) } else { 0 };
        if sake_min == 0 {
            return false;
        }
        // 上位m個の中で確実に日本酒が入る下位sake_min個の合計
        // 大きい順なのでインデックス[m-sake_min, m)の合計
        let sum = prefix[m] - prefix[m - sake_min];
        sum >= x
    };

    let mut lo = 1usize;
    let mut hi = n;
    let mut res = -1i64;

    // にぶたん
    if check(n) {
        res = n as i64;
        while lo <= hi {
            let mid = (lo + hi) / 2;
            if check(mid) {
                res = mid as i64;
                hi = mid - 1;
            } else {
                lo = mid + 1;
            }
        }
    }
    println!("{}", res);
}
