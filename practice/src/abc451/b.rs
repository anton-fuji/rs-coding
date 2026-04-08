use proconio::{fastout, input, marker::Chars};

#[fastout]
#[allow(unused_imports)]
fn main() {
    input! {
        n: usize,
        m :usize
    }

    let mut g_now = vec![0i64; m + 1];
    let mut g_next = vec![0i64; m + 1];

    for _ in 0..n {
        input! {a: usize, b:usize}

        g_now[a] += 1;
        g_next[b] += 1;
    }

    for i in 1..=m {
        let res = g_next[i] - g_now[i];
        println!("{}", res);
    }
}
