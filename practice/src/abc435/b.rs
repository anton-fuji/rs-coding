use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
      n : usize,
      a: [usize; n]
    }

    let mut count = 0;
    for l in 0..n {
        let mut sum = 0;
        for r in l..n {
            sum += a[r];
            let ok = (l..=r).all(|f| sum % a[f] != 0);
            if ok {
                count += 1;
            }
        }
    }
    println!("{count}");
}
