use proconio::{fastout, input, marker::Chars};

#[fastout]
#[allow(unused_imports, non_snake_case)]
fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n]
    }

    for i in n - k..n {
        print!("{} ", a[i]);
    }
    for j in 0..n - k {
        print!("{} ", a[j]);
    }

    println!();
}
