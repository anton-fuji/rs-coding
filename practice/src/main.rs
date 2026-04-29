use proconio::{fastout, input, marker::Chars};

#[fastout]
#[allow(unused_imports, non_snake_case)]
fn main() {
    input! {
        s: String,
    }

    if s.ends_with("tea") {
        println!("Yes");
    } else {
        println!("No");
    }
}
