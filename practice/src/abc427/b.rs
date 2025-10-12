use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
       s: usize,
    }

    let mut a = vec![0_usize; s + 1];
    a[0] = 1;

    for i in 0..=s {
        for j in 0..i {
            a[i] += solve(a[j]);
        }
    }

    println!("{}", a[s]);
}

fn solve(a: usize) -> usize {
    let mut res = 0;
    for c in a.to_string().chars() {
        res += c.to_digit(10).unwrap() as usize;
    }

    res
}
