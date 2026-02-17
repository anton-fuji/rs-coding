use proconio::{fastout, input};

#[fastout]
#[allow(unused_imports)]
fn main() {
    input! {
        s :String,
    }
    let res = format!("{}s", s);
    println!("{res}");
}
