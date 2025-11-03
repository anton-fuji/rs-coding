use std::f64::consts::PI;

use itertools::Itertools;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
      n: usize,
      xy: [(f64, f64);n]
    }

    let mut diff = f64::INFINITY;
    for i in 0..n {
        let (x1, y1) = xy[i];
        let tilt = (0..i)
            .chain(i + 1..n)
            .map(|j| xy[j])
            .map(|(x2, y2)| (y2 - y1).atan2(x2 - x1))
            .sorted_unstable_by(|a, b| a.total_cmp(b))
            .collect_vec();

        let mut r = 1;
        for l in 0..n - 2 {
            while r < n - 2 && tilt[r] < tilt[l] + PI {
                r += 1;
            }

            let diff_1 = tilt[r] - tilt[l] - PI;
            let diff_2 = tilt[r - 1] - tilt[l] - PI;
            diff = diff.min(diff_1.abs()).min(diff_2.abs())
        }
    }

    let res = (1.0 - diff / PI) * 180.0;
    println!("{res}");
}
