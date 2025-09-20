use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
      n: usize,
      p: [usize; n]
    }

    let mut count = vec![0; 101];
    for &p in &p {
        count[p] += 1;
    }

    let mut t = 0;
    for c in count.iter_mut().rev() {
        (*c, t) = (t + 1, t + *c);
    }
    for p in p {
        println!("{}", count[p]);
    }
}
