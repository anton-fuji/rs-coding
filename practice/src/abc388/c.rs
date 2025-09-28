use proconio::input;

fn main() {
    input! {
     n: usize,
     a: [usize; n]
    }

    let mut res = 0;
    for &ai in a.iter() {
        let idx = binary_serch(&a, ai * 2);
        res += n - idx - 1;
    }
    println!("{res}");
}

fn binary_serch(a: &[usize], x: usize) -> usize {
    let mut l = 0;
    let mut r = a.len();
    while (r - l) > 1 {
        let m = (l + r) / 2;
        if a[m] < x {
            l = m;
        } else {
            r = m;
        }
    }
    l
}
