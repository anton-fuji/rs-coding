use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
      n: usize,
      a: [usize; n],
    }

    let mut res = "Yes";
    for i in 0..n - 1 {
        if a[i] > a[i + 1] || a[i] == a[i + 1] {
            res = "No";
            break;
        }
    }
    println!("{res}");
}
