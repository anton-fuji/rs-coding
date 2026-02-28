use proconio::{fastout, input, marker::Chars};

#[fastout]
#[allow(unused_imports)]
fn main() {
    input! {
        s: String,
    }

    let mut c = s.chars();
    let hw = c.next().unwrap().to_lowercase();
    let rest = c.collect::<String>();
    let l = hw.collect::<String>() + &rest;

    let res = format!("Of{}", l);
    println!("{}", res);
}
