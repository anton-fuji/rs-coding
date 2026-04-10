use proconio::{fastout, input, marker::Chars};

#[fastout]
#[allow(unused_imports)]
fn main() {
    input! {
        h: usize,
        w :usize
    }

    for i in 0..h {
        for j in 0..w {
            if i == 0 || j == 0 || i == h - 1 || j == w - 1 {
                print!("#");
            } else {
                print!(".")
            }
        }
        println!();
    }
}
