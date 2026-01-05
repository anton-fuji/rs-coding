use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
       w:usize,
       b:usize,
    }

    let res = (w * 1000) / b + 1;
    println!("{res}");
}
