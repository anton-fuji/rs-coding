use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
      mut n: usize,
      a: [usize; n],
    }

    n = n - 2;
    let mut res = "No";
    for i in 0..n {
        if a[i] == a[i + 1] && a[i + 1] == a[i + 2] {
            res = "Yes";
        }
    }
    println!("{res}");
}
