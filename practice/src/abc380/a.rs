use proconio::marker::Chars;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: Chars,
    }

    let mut cnt = vec![0; 3];
    let judge = [1, 2, 3];
    for c in &n {
        match c {
            '1' => {
                cnt[0] += 1;
            }
            '2' => {
                cnt[1] += 1;
            }
            '3' => {
                cnt[2] += 1;
            }
            _ => {}
        }
    }
    if cnt == judge {
        println!("Yes");
    } else {
        println!("No");
    }
}
