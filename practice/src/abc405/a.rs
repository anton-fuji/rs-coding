use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
      r: i64,
      x: i64,
    }

    match x {
        1 => {
            if 1600 <= r && 2999 >= r {
                println!("Yes");
            } else {
                println!("No");
            }
        }
        2 => {
            if 1200 <= r && 2399 >= r {
                println!("Yes");
            } else {
                println!("No");
            }
        }
        _ => {
            return;
        }
    }
}
