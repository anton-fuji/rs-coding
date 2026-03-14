use proconio::{fastout, input, marker::Chars};

#[fastout]
#[allow(unused_imports)]
fn main() {
    input! {
        n: usize,
        mut x: usize,
        a: [usize; n]
    }

    for ai in a {
        let mut tmp = 0;
        if ai < x {
            x = ai;
            tmp = 1;
        }
        println!("{}", tmp);
    }
}
