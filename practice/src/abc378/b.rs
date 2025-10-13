use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
     n: usize,
      qr: [(i64, i64); n],
      query: usize,
      td: [(usize, i64); query],
    }

    for (t, d) in td {
        let (q, r) = qr[t - 1];
        let res = d + (q + r - d % q) % q;
        println!("{res}");
    }
}
