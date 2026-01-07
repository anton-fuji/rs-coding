use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
      t: usize
    }

    for _ in 0..t {
        input! {
          (n, h): (usize, usize),
        }
        let mut curr = 0;
        let mut min = h;
        let mut max = h;
        let mut ok = true;

        for _ in 0..n {
            input! {
              (t, l, u) : (usize, usize, usize)
            }
            // 上方向にいける最大高度(max + dt)
            max = (max + t - curr).min(u);
            // 0未満になったら0に張り付く
            min = min.saturating_sub(t - curr).max(l);
            ok &= max >= l && min <= u;
            curr = t;
        }
        if ok {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}
