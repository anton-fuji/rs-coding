use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
      n: usize,
       dist: [usize; n-1]
    }

    for i in 0..n - 1 {
        let mut tmp = 0;
        for j in i..n - 1 {
            tmp += dist[j];
            print!("{tmp} ");
        }
        println!();
    }
}
