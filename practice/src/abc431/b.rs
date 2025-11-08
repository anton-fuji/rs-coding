use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
      x: usize,
      n: usize,
      w: [usize; n],
      q: usize,
      p: [usize; q],
    }

    let mut rob_w = x;
    let mut reg = vec![false; n];

    for &pi in &p {
        let t = pi - 1;
        if reg[t] {
            rob_w -= w[t];
            reg[t] = false;
        } else {
            rob_w += w[t];
            reg[t] = true;
        }
        println!("{rob_w}");
    }
}
