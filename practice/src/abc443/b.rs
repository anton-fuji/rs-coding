use proconio::{fastout, input, marker::Chars};

#[fastout]
#[allow(unused_imports)]
fn main() {
    input! {
        mut n: usize,
        k: usize
    }
    let mut sum = 0;
    let mut y = 0;

    loop {
        sum += n;
        if sum >= k {
            println!("{}", y);
            break;
        }
        n += 1;
        y += 1;
    }
}
