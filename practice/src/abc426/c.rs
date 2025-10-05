use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
      n: usize,
      q: usize,
      xy: [(usize, usize); q]
    }

    let mut res = vec![1; n + 1];
    let mut low = 1usize;

    for (x, y) in &xy {
        if *x < low {
            println!("0");
            continue;
        }
        let mut count = 0;
        for i in low..=*x {
            count += res[i];
        }
        res[*y] += count;
        println!("{count}");

        low = low.max(*x + 1);
    }
}
