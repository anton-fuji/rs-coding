use proconio::input;

fn main() {
    solve();
}

fn solve() {
    input! {
        n: usize,
        tv: [(usize, usize); n],
    }
    let mut ans = 0;
    let mut last_time = 0;
    for (t, v) in tv.into_iter() {
        if ans + last_time < t {
            ans = 0;
        } else {
            ans -= t - last_time
        };
        last_time = t;
        ans += v
    }
    println!("{}", ans);
}
