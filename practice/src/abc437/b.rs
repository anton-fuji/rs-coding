use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
      h: usize,
      w: usize,
      n: usize,
      a: [[i32; w]; h],
      b: [i32; n]
    }

    let res = a
        .iter()
        .map(|f| f.iter().filter(|&x| b.contains(x)).count())
        .max()
        .unwrap();
    println!("{res}");
}
