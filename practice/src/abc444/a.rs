use proconio::{fastout, input, marker::Chars};

#[fastout]
#[allow(unused_imports)]
fn main() {
    input! {
     n: Chars
    }
    let mut flag = false;
    if n[0] == n[1] && n[1] == n[2] {
        flag = true;
    }

    println!("{}", if flag { "Yes" } else { "No" })
}
