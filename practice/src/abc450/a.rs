use proconio::{fastout, input, marker::Chars};

#[fastout]
#[allow(unused_imports)]
fn main() {
    input! {
        n: usize,
    }
    let res = (1..=n)
        .rev()
        .map(|i| i.to_string())
        .collect::<Vec<_>>()
        .join(",");
    println!("{}", res);
}
