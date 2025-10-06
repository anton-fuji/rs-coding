use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
      _n: usize,
    mut d: usize,
    mut  s:Chars
    }

    for c in s.iter_mut().rev() {
        if d == 0 {
            break;
        }
        if *c == '@' {
            *c = '.';
            d -= 1;
        }
    }
    println!("{}", s.iter().collect::<String>());
}
