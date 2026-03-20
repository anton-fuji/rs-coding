use proconio::{fastout, input, marker::Chars};

#[fastout]
#[allow(unused_imports)]
fn main() {
    input! {
        n: usize,
        a: Chars,
        b: Chars
    }

    let mut flag = false;
    for i in 0..n {
        if a[i] == 'o' && b[i] == 'o' {
            flag = true;
            break;
        }
    }
    println!("{}", if flag { "Yes" } else { "No" });
}
