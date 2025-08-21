use proconio::{input, marker::Chars};

fn main() {
    input! {
      s: Chars,
    }

    let mut ans: f64 = 0.0;

    for i in 1..s.len() {
        for j in i + 2..s.len() {
            let t = &s[i..=j];

            if t.len() < 3 {
                continue;
            }

            if t[0] == 't' && t[t.len() - 1] == 't' {
                let x = t.iter().filter(|&c| *c == 't').count();
                ans = ans.max((x - 2) as f64 / (t.len() - 2) as f64);
            }
        }
    }
    println!("{:.15}", ans);
}
