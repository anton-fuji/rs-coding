use proconio::{fastout, input, marker::Chars};

#[fastout]
#[allow(unused_imports)]
fn main() {
    input! {
        m: usize,
        d: usize
    }

    match (m, d) {
        (1, 7) => println!("Yes"),
        (3, 3) => println!("Yes"),
        (5, 5) => println!("Yes"),
        (7, 7) => println!("Yes"),
        (9, 9) => println!("Yes"),
        (_, _) => println!("No"),
    }
}
