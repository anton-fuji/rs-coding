use proconio::{fastout, input, marker::Chars};

#[fastout]
#[allow(unused_imports, non_snake_case)]
fn main() {
    input! {
        n:usize,
        k:usize,
        x:usize,
        mut a:[usize;n],
    }
    a.sort();
    let sake = &a[0..k];
    let mut sake = sake.to_vec();
    sake.reverse();
    let mut counter = 0;
    let mut res: isize = -1;
    for i in 0..k {
        counter += sake[i];
        if counter >= x {
            res = (i + 1 + (n - k)) as isize;
            break;
        }
    }
    println!("{}", res);
}
