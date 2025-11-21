use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
      s: Chars,
    }

    let l = s.len();
    let new_s: String = s.iter().skip(l - 3).take(l).collect();
    let mut res = "No";
    if new_s == "san" {
        res = "Yes";
    }
    println!("{res}");
}
