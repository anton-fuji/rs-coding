use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
        s: Chars,
    }
    println!(
        "{}",
        s.iter().filter(|c| c.is_ascii_digit()).collect::<String>()
    );
}
