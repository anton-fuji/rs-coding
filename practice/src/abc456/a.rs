use proconio::{fastout, input, marker::Chars};

#[fastout]
#[allow(unused_imports, non_snake_case)]
fn main() {
    input! {
        x: usize,
    }

    for i in 1..=6 {
        for j in 1..=6 {
            for k in 1..=6 {
                if x == i + j + k {
                    println!("Yes");
                    return;
                }
            }
        }
    }
    println!("No");
}
