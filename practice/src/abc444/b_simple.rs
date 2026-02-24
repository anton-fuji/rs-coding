use proconio::{fastout, input, marker::Chars};

#[fastout]
#[allow(unused_imports)]
fn main() {
    input! {
     n: usize,
     k: usize,
    }

    let mut cnt = 0;
    for c in 1..=n {
        let sum: usize = c
            .to_string()
            .chars()
            .map(|ch| ch.to_digit(10).unwrap() as usize)
            .sum();
        if sum == k {
            cnt += 1;
        }
    }
    println!("{cnt}");
}
