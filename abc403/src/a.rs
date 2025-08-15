use proconio::input;

fn main() {
    input! {
      n: usize,
      a: [i32; n],
    }

    let mut sum = 0;
    for i in (0..n).step_by(2) {
        sum += a[i];
    }

    println!("{sum}")
}
