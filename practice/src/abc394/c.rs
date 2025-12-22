use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {mut s: Chars}

    let n = s.len();
    for i in (0..n - 1).rev() {
        if s[i] == 'W' && s[i + 1] == 'A' {
            s[i] = 'A';
            s[i + 1] = 'C';
        }
    }
    println!("{}", s.iter().collect::<String>());
}
