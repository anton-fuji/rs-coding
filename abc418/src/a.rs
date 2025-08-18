use proconio::{input, marker::Chars};

fn main() {
    input! {
      n: usize,
      s: Chars,
    }
    if n < 3 {
        println!("No");
        return;
    }

    if s[n - 3..] == ['t', 'e', 'a'] {
        println!("Yes");
    } else {
        println!("No");
    }
}
