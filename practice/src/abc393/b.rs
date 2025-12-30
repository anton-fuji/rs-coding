use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
      s:Chars
    }

    let mut cnt = 0;
    for i in 0..s.len() {
        for j in 0..s.len() {
            for k in 0..s.len() {
                if i >= j || j >= k {
                    continue;
                }
                if j - i != k - j {
                    continue;
                }
                if s[i] != 'A' {
                    continue;
                }
                if s[j] != 'B' {
                    continue;
                }
                if s[k] != 'C' {
                    continue;
                }
                cnt += 1;
            }
        }
    }
    println!("{cnt}");
}
