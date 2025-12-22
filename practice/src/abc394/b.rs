use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {n:usize,mut s: [String; n]}

    s.sort_by_key(|f| f.len());
    println!("{}", s.join(""));
}
