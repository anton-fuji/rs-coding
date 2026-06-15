use proconio::{fastout, input, marker::Chars};

#[fastout]
#[allow(unused_imports, non_snake_case)]
fn main() {
    input! {
        n: usize,
        a: [usize; n],
        b: [usize; n],
    }

    let mut v = vec![0; n];
    for i in 0..n {
        let bi = b[i];
        v[bi - 1] += i + 1;
    }

    println!("{}", if a == v { "Yes" } else { "No" });
}
