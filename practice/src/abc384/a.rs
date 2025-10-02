use proconio::{input, marker::Chars};

fn main() {
    input! {
        _n:usize,
       c1: char,
      c2: char,
      s: Chars
    }

    let res: String = s.iter().map(|c| if *c == c1 { &c } else { &c2 }).collect();
    println!("{res}");
}
