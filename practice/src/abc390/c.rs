use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
      h: usize,
      w: usize,
      s: [Chars; h],
    }

    let mut l = usize::MAX;
    let mut r = usize::MIN;
    let mut t = usize::MAX;
    let mut b = usize::MIN;

    for i in 0..h {
        for j in 0..w {
            if s[i][j] == '#' {
                l = l.min(j);
                r = r.max(j);
                t = t.min(i);
                b = b.max(i);
            }
        }
    }

    for i in t..=b {
        for j in l..=r {
            if s[i][j] == '.' {
                println!("No");
                return;
            }
        }
    }
    println!("Yes");
}
