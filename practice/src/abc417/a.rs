use proconio::input;

fn main() {
    input! {
      n: usize,
      a: usize,
      b: usize,
      mut s: String,
    }

    // 指定範囲を空文字に置き換える
    s.replace_range(n - b..n, "");
    s.replace_range(0..a, "");
    println!("{}", s);
}
