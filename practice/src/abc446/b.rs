use proconio::{fastout, input, marker::Chars};

#[fastout]
#[allow(unused_imports)]
fn main() {
    input! {
        n: usize,
        m: usize,
    }

    let mut used = vec![false; m + 1];
    for _ in 0..n {
        input! {
            l: usize,
            x: [usize; l]
        }

        let mut ng = false;
        for i in x {
            if !used[i] {
                used[i] = true;
                println!("{}", i);
                ng = true;
                break;
            }
        }
        if !ng {
            println!("0");
        }
    }
}
