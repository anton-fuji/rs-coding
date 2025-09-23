use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
       n: i32,
    }

    for i in 1..=n {
        for j in 1..=n {
            let layer = i.min(n - i + 1).min(j.min(n - j + 1));

            if layer % 2 == 1 {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}
