use proconio::{fastout, input, marker::Chars};

#[fastout]
#[allow(unused_imports, non_snake_case)]
fn main() {
    input! {
        n: usize,
        a: [usize; n],
        x: usize
    }

    println!("{}", a[x - 1]);
}
