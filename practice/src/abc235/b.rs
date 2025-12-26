use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
       n: usize,
      h: [usize; n],
    }

    let mut res = h[0];
    for i in 1..n {
        if res < h[i] {
            res = h[i]
        } else {
            break;
        }
    }
    println!("{res}");
}
