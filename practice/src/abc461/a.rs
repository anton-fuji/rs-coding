use proconio::{fastout, input, marker::Chars};

#[fastout]
#[allow(unused_imports, non_snake_case)]
fn main() {
    input! {
        a: usize,
        d: usize
    }

    if a <= d {
        println!("Yes");
    } else {
        println!("No");
    }
}
