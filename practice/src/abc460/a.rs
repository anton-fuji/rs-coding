use proconio::{fastout, input, marker::Chars};

#[fastout]
#[allow(unused_imports, non_snake_case)]
fn main() {
    input! {
        n: usize,
        mut m: usize,
    }
    let mut x = 0;
    let mut cnt = 0;
    while m > 0 {
        x = n % m;
        m = x;
        if x != 0 {
            cnt += 1;
        }
    }
    println!("{}", cnt + 1);
}
