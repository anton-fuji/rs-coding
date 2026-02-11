use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
      _n: usize,
      _m: usize,
      s: String,
      t: String,
      q :usize,
    }

    let mut smask = 0u32;
    for c in s.chars() {
        smask |= 1 << (c as u8 - b'a');
    }

    let mut tmask = 0u32;
    for c in t.chars() {
        tmask |= 1 << (c as u8 - b'a');
    }

    for _ in 0..q {
        input! {w: String}

        let mut wmask = 0u32;
        for c in w.chars() {
            wmask |= 1 << (c as u8 - b'a');
        }

        let in_s = wmask & !smask == 0;
        let in_t = wmask & !tmask == 0;

        if in_s && !in_t {
            println!("Takahashi");
        } else if in_t && !in_s {
            println!("Aoki");
        } else {
            println!("Unknown");
        }
    }
}
