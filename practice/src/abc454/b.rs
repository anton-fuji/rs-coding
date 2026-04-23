use proconio::{fastout, input, marker::Chars};

#[fastout]
#[allow(unused_imports, non_snake_case)]
fn main() {
    input! {
        n: usize,
        m: usize,
        f: [usize; n]
    }

    let mut cnt = vec![0usize; m + 1];

    for &v in f.iter() {
        cnt[v] += 1;
    }

    let flg1 = cnt.iter().all(|&x| x <= 1);
    let flg2 = cnt[1..=m].iter().all(|&x| x >= 1);
    println!("{}", if flg1 { "Yes" } else { "No" });
    println!("{}", if flg2 { "Yes" } else { "No" });
}
