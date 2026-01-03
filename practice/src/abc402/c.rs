use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
      n:usize,
      m:usize,
      a: [[usize]; m],
    }
    let mut b = vec![0; n + 1];
    for i in 0..n {
        input! {a:usize}
        b[a] = i;
    }
    let mut v = vec![0; n];
    for i in a.iter().map(|a| a.iter().map(|&a| b[a]).max().unwrap()) {
        v[i] += 1;
    }

    let mut sum = 0;
    for i in v {
        sum += i;
        println!("{sum}");
    }
}
