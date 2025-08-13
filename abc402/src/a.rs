use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
      s: Chars,
    }

    let res = s
        .into_iter()
        .filter(|c| c.is_uppercase())
        .collect::<String>();

    println!("{}", res);
}
