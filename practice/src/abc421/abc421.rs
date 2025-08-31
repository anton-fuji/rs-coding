use proconio::input;

fn main() {
    input! {
      n: i64,
      s: [String;n],
      x: usize,
      y: String,
    }

    if s[x - 1] == y {
        println!("Yes");
    } else {
        println!("No");
    }
}
