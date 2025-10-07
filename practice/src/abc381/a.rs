use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: usize,
        s: Chars,
    }

    let mut ok = true;

    let l = s.len();
    if l % 2 == 1 && s[l / 2] == '/' {
        for i in 0..(l + 1) / 2 - 1 {
            if s[i] != '1' {
                ok = false;
            }
        }

        for i in (l + 1) / 2..n {
            if s[i] != '2' {
                ok = false;
            }
        }
    } else {
        ok = false;
    }

    println!("{}", if ok { "Yes" } else { "No" });
}
