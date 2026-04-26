use proconio::{fastout, input, marker::Chars};

#[fastout]
#[allow(unused_imports, non_snake_case)]
fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize,
    }

    println!("{}", if a!= b && b == c {"Yes"}else{"No"});
}
