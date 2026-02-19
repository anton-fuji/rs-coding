use proconio::{fastout, input};

#[fastout]
#[allow(unused_imports)]
fn main() {
    input! {
        n: u32
    }
    println!("{}", (2u32.pow(n) - (2 * n)));
}
