use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
      a: [usize; 7]
    }

    let mut v = vec![0; 20];
    for &a in a.iter() {
        v[a] += 1;
    }
    let mut t = 0;
    let mut tr = 0;
    for &v in v.iter() {
        if v >= 2 {
            t += 1;
        }
        if v >= 3 {
            tr += 1;
        }
    }
    println!("{}", if t >= 2 && tr >= 1 { "Yes" } else { "No" });
}
