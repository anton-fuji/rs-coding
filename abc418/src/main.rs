use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    }

    let mut ans: f64 = 0.0;

    for i in 0..s.len() {
        for j in i + 2..s.len() {
            if s[i] == 't' && s[j] == 't' {
                let x = s[i..=j].iter().filter(|&&c| c == 't').count();
                ans = ans.max((x - 2) as f64 / (j - i) as f64);
            }
        }
    }

    println!("{:.15}", ans);
}
