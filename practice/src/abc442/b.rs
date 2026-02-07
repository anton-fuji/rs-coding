use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
      q: usize
    }

    let mut vol = 0;
    let mut s = false;
    for _ in 0..q {
        input! {
            a: usize
        }

        match a {
            1 => {
                vol += 1;
            }
            2 => {
                if vol >= 1 {
                    vol -= 1;
                }
            }
            3 => {
                if s {
                    s = false;
                } else {
                    s = true;
                }
            }
            _ => {}
        }
        if vol >= 3 && s {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}
