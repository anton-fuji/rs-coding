use proconio::{fastout, input, marker::Chars};

#[fastout]
#[allow(unused_imports, non_snake_case)]
fn main() {
    input! {
        s: String,
        n:usize,
    }

    let res = s.chars().skip(n).collect::<String>();
    println!("{}", &res[..res.len() - n]);
}
