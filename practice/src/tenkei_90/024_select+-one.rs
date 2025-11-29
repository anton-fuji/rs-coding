use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
      (n, k ) : (usize, u64),
      a: [u64; n],
      b: [u64; n],
    }

    let mut k = Some(k);

    for (&ai, &bi) in a.iter().zip(b.iter()) {
        let diff = ai.abs_diff(bi);
        if let Some(remainig) = k {
            k = remainig.checked_sub(diff);
        } else {
            break;
        }
    }

    if let Some(l) = k {
        if l % 2 == 0 {
            println!("Yes");
        } else {
            println!("No");
        }
    } else {
        println!("No");
    }
}
