use proconio::{fastout, input, marker::Chars};

#[fastout]
#[allow(unused_imports, non_snake_case)]
fn main() {
    input! {
        l: i64,
        r: i64
    }
    println!("{}", r-l+1);
}
