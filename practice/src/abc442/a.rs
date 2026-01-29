use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
      s: Chars
    }

    let cnt = s.iter().filter(|&&c| c == 'i' || c == 'j').count();
    println!("{cnt}");
}
