use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
      s: Chars
    }

    let res: usize = (0..s.len() - 1)
        .filter(|&i| s[i] as u8 + 1 == s[i + 1] as u8)
        .map(|i| {
            let l = s[..=i].iter().rev().take_while(|&&c| c == s[i]).count();
            let r = s[i + 1..].iter().take_while(|&&c| c == s[i + 1]).count();
            l.min(r)
        })
        .sum();

    println!("{res}");
}
