use proconio::{input, marker::Chars};

fn main() {
    input! {
     s: Chars,
    }

    let mut count = 0;
    let mut i = 0;
    let n = s.len();
    while i < n {
        count += 1;
        i += if i + 1 < n && s[i] == '0' && s[i + 1] == '0' {
            2
        } else {
            1
        };
    }
    println!("{count}");
}
