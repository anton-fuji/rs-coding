use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
      n :usize,
      a: [usize; n]
    }

    for i in 0..n {
        let arr = &a[..i];
        let res = arr.iter().enumerate().rev().find(|&(_, v)| *v > a[i]);

        match res {
            Some((idx, _v)) => println!("{}", idx + 1),
            None => println!("-1"),
        }
    }
}
