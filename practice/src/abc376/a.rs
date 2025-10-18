use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
      n: usize,
      c: usize,
      t: [usize; n]
    }

    let mut count = 1;
    let mut sub_t = 0;
    let mut tmp = t[0];
    for i in 1..n {
        sub_t = t[i] - tmp;
        if sub_t >= c {
            count += 1;
            tmp = t[i];
        }
    }
    println!("{count}");
}
