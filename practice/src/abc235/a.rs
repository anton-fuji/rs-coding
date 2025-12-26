use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
      mut n: usize,
    }

    let a = n / 100;
    let b = (n / 10) % 10;
    let c = n % 10;
    let bca = b * 100 + c * 10 + a;
    let cab = c * 100 + a * 10 + b;
    let res = n + bca + cab;
    println!("{res}");
}
