use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
      x: u32,
      y: u32,
    }

    let a = 2_i32.pow(y);
    println!("{}", x as i32 * a);
}
