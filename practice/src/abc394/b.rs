use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
       n: usize,
     mut  s: [String; n],
    }

    s.sort_by(|a, b| a.len().cmp(&b.len()));
    let res = s.join("");
    println!("{res}");
}
