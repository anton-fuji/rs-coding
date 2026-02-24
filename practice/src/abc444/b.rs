use proconio::{fastout, input, marker::Chars};

#[fastout]
#[allow(unused_imports)]
fn main() {
    input! {
     n: usize,
     k: usize,
    }

    let mut cnt = 0;
    for mut c in 1..=n {
        let mut sum = 0;
        while c > 0 {
            sum += c % 10;
            c /= 10;
        }
        if sum == k {
            cnt += 1;
        }
    }
    println!("{cnt}");
}
