use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
    n: usize,
    }

    for i in 0..n {
        let mut row = String::with_capacity(n);
        for j in 0..n {
            let d = i.min(j).min(n - 1 - i).min(n - 1 - j);
            row.push(if d % 2 == 0 { '#' } else { '.' });
        }
        println!("{}", row);
    }
}
