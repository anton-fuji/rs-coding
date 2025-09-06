use proconio::{input, marker::Chars};

fn main() {
    input! {
       s: Chars,
    }

    let mut res = Vec::new();
    for i in 0..s.len() {
        if s[i] == '#' {
            res.push(i + 1);
        }
    }

    for i in (0..res.len()).step_by(2) {
        println!("{},{}", res[i], res[i + 1]);
    }
}
