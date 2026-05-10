use proconio::{fastout, input, marker::Chars};

#[fastout]
#[allow(unused_imports, non_snake_case)]
fn main() {
    input! {
        n: usize,
    }

    let mut a: Vec<Vec<usize>> = Vec::new();
    for _ in 0..n {
        input! {l: usize, row: [usize; l]}
        a.push(row);
    }
    input! {x:usize, y:usize}
    println!("{}", a[x - 1][y - 1]);
}
