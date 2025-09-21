use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
      n: usize,
      m: usize,
      k: usize,
      ab: [(usize, usize); k],
    }

    let mut count = vec![0; n];

    for (a, _) in ab {
        count[a - 1] += 1;

        if count[a - 1] == m {
            print!("{a} ");
        }
    }
    println!();
}
