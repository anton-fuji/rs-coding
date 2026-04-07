use proconio::{fastout, input, marker::Chars};

#[fastout]
#[allow(unused_imports)]
fn main() {
    input! {
        h: usize,
        w: usize,
        q: usize,
    }

    let mut curr_h = h;
    let mut curr_w = w;

    for _ in 0..q {
        input! {t: usize, n: usize}

        if t == 1 {
            println!("{}", n * curr_w);
            curr_h -= n;
        } else {
            println!("{}", n * curr_h);
            curr_w -= n;
        }
    }
}
