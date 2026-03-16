use proconio::{fastout, input, marker::Chars};

#[fastout]
#[allow(unused_imports)]
fn main() {
    input! {
        n: usize,
        m: usize,
        c: [usize; m],
    }

    let mut p = vec![0; m + 1];
    for _ in 0..n {
        input! {a: usize, b: usize}
        p[a - 1] += b;
    }

    let mut res = 0;
    for j in 0..m {
        res += p[j].min(c[j]);
    }
    println!("{}", res);
}
