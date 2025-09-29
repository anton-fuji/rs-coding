use proconio::*;

#[fastout]
fn main() {
    input! {
        n: usize,
    l: [usize; n]
    }
    println!("{}", solve(n, &l))
}

fn solve(n: usize, l: &[usize]) -> usize {
    let mut first = 1 << 60;
    let mut last = 0;
    for i in 0..n {
        if l[i] == 0 {
            continue;
        }
        first = first.min(i + 1);
        last = last.max(i + 1);
    }
    if last == 0 {
        return 0;
    }
    last - first
}
