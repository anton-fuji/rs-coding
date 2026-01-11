use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
      t: usize,
    }

    for _ in 0..t {
        input! {
          n: usize,
          wp: [(usize, usize); n]
        }
        let mut cost = Vec::with_capacity(n);
        let mut cap = 0;
        for (w, p) in wp {
            cap += p;
            cost.push(w + p);
        }
        cost.sort();
        let mut used = 0;
        let mut res = 0;
        for c in cost {
            if used + c > cap {
                break;
            } else {
                used += c;
                res += 1;
            }
        }
        println!("{res}");
    }
}
