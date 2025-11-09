use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
      n: usize,
      m: usize,
      k: usize,
      mut h: [usize; n],
      mut b: [usize; m],
    }

    h.sort_unstable();
    b.sort_unstable();

    let mut matches = 0;
    let mut body_idx = 0;

    for &head in &h {
        while body_idx < m && b[body_idx] < head {
            body_idx += 1;
        }

        if body_idx < m {
            matches += 1;
            body_idx += 1;

            if matches >= k {
                println!("Yes");
                return;
            }
        }
    }
    println!("No");
}
