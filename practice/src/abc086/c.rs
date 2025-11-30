use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
      n: usize,
    }

    let mut pt = 0;
    let mut px = 0;
    let mut py = 0;

    for _ in 0..n {
        input! {t: u64, x: u64, y:u64}
        let dt = t - pt;
        let dist = x.abs_diff(px) + y.abs_diff(py);

        // 距離が時間より大きい
        if dist > dt {
            println!("No");
            return;
        }

        // 時間 - 距離の偶奇が一致しない→　無視
        if (dt - dist) % 2 != 0 {
            println!("No");
            return;
        } else {
            pt = t;
            px = x;
            py = y;
        }
    }

    println!("Yes");
}
