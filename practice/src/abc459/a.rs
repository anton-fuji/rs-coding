use proconio::{fastout, input, marker::Chars};

#[fastout]
#[allow(unused_imports, non_snake_case)]
fn main() {
    input! {
        x:usize,
    }
    let s = "HelloWorld";
    let res: String = s
        .chars()
        .enumerate()
        .filter(|&(i, _)| i != x - 1)
        .map(|(_, c)| c)
        .collect();
    println!("{}", res)
}
