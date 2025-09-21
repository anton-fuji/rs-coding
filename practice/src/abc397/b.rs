use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
      s: String,
    }

    let mut count = 0;
    for (i, c) in s.chars().enumerate() {
        let expected = if (i + count) % 2 == 0 { 'i' } else { 'o' };
        if c != expected {
            count += 1;
        }
    }
    if (s.chars().count() + count) % 2 == 1 {
        count += 1;
    }
    println!("{count}");
}
