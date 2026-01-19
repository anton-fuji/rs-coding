use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
      n: usize,
      m: usize,
      s: Chars,
      t: Chars,
    }

    let mut res = i32::MAX;
    for l in 0..=n - m {
        let mut c = 0;
        for i in 0..m {
            let s_digit = s[l + i] as i32 - '0' as i32;
            let t_digit = t[i] as i32 - '0' as i32;

            c += (s_digit - t_digit + 10) % 10;
        }
        res = res.min(c);
    }
    println!("{res}");
}
