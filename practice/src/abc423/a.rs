use proconio::*;

#[fastout]
fn main() {
    input! {
        x: usize,
    c: usize
    }

    let res = x / (1000 + c);
    println!("{}", res * 1000);
}
