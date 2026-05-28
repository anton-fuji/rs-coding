use proconio::{fastout, input, marker::Chars};

#[fastout]
#[allow(unused_imports, non_snake_case)]
fn main() {
    input! {
        a: usize,
        b: usize
    }

    if a == b {
        println!("1");
        return;
    }

    if (a + b) % 2 == 0 {
        println!("3");
    } else {
        println!("2");
    }
}
