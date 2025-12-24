use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {n:usize, a: [usize; n]}

    let mut last = vec![None; 1_000_001];
    let mut res = usize::MAX;
    for (i, &x) in a.iter().enumerate() {
        if let Some(prev) = last[x] {
            res = res.min(i - prev + 1);
        }
        last[x] = Some(i);
    }
    if res == usize::MAX {
        println!("-1");
    } else {
        println!("{res}");
    }
}
