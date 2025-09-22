use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
     n: usize,
      a: [usize; n],
    }

    let mut res = "No";
    for i in 0..n - 1 {
        if a[i] < a[i + 1] {
            res = "Yes";
        } else {
            res = "No";
            println!("{res}");
            return;
        }
    }
    println!("{res}");
}
