use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
      mut n: usize,
      s: Chars,
    }

    let mut count = 0;
    if s.len() == 1 {
        println!("0");
        return;
    }
    for i in 0..n - 2 {
        if s[i] == '#' && s[i + 2] == '#' {
            if s[i + 1] == '.' {
                count += 1;
            }
        }
    }
    println!("{count}");
}
