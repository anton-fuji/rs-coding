use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
      t: Chars,
      u: Chars,
    }

    if solve(t, u) {
        println!("Yes");
    } else {
        println!("No");
    }
}

fn solve(t: Vec<char>, u: Vec<char>) -> bool {
    'outer: for d in 0..=t.len() - u.len() {
        for i in 0..u.len() {
            if t[i + d] != '?' && t[i + d] != u[i] {
                continue 'outer;
            }
        }
        return true;
    }
    false
}
