use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
       rc: [(usize, usize); n]
    }

    let max_x = rc.iter().map(|(x, _)| x).max().unwrap();
    let min_x = rc.iter().map(|(x, _)| x).min().unwrap();
    let max_y = rc.iter().map(|(_, y)| y).max().unwrap();
    let min_y = rc.iter().map(|(_, y)| y).min().unwrap();

    println!("{}", ((max_x - min_x).max(max_y - min_y) + 1) / 2);
}
