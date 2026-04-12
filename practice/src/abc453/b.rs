use proconio::{fastout, input, marker::Chars};

#[fastout]
#[allow(unused_imports)]
fn main() {
    input! {
        t: usize,
        x: usize,
        a: [usize; t+1]
    }

    let mut carry = a[0];
    println!("0 {}", carry);
    for i in 1..=t {
        if carry.abs_diff(a[i]) >= x {
            carry = a[i];
            print!("{} {}", i, carry);
            println!();
        }
    }
}
