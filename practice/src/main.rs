use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
      n: usize,
      l: usize,
      k:usize,
      mut a:[usize; n]
    }

    a.push(l);
    let mut left = 0;
    let mut right = l;

    while right - left > 1 {
        let mid = (left + right) / 2;
        if can(mid, k, &a) {
            left = mid; // もっと大きくできる
        } else {
            right = mid; // 小さくする
        }
    }
    println!("{left}");
}

fn can(x: usize, k: usize, a: &Vec<usize>) -> bool {
    let mut cnt = 0;
    let mut pre = 0;

    for &pos in a.iter() {
        if pos - pre >= x {
            cnt += 1;
            pre = pos;
        }
    }
    cnt >= k + 1
}
