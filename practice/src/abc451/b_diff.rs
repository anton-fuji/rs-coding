use proconio::{fastout, input, marker::Chars};

#[fastout]
#[allow(unused_imports)]
fn main() {
    input! {
        n: usize,
        m :usize
    }

    let mut diff = vec![0i64; m + 1];
    for _ in 0..n {
        input! {a:usize, b:usize}
        diff[a] -= 1;
        diff[b] += 1;
    }
    for i in 1..=m {
        println!("{}", diff[i]);
    }
}

