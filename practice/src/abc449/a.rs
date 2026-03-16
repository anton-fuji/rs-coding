use proconio::{fastout, input, marker::Chars};

#[fastout]
#[allow(unused_imports)]
fn main() {
    input! {
        d: f64,
    }
    let dd = d / 2.0;

    println!("{}", dd * dd * 3.14159265);
}
