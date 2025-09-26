use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        x:usize,
    }

    let mut n = 1;
    let mut bai = 1;
    loop {
        if x == bai {
            break;
        }
        n += 1;
        bai *= n;
    }
    println!("{n}");
}
