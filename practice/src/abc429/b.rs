use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
      n: usize,
      m: usize,
      a: [usize; n]
    }

    let mut res = "No";
    let sum: usize = a.iter().sum();
    let sub = sum - m;
    for i in 0..n {
        if a[i] == sub {
            res = "Yes";
            break;
        }
    }
    println!("{res}");
}
