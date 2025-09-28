use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [isize; n],
    }

    for p in (1..=n).permutations(n) {
        if a.iter().zip(&p).all(|(&a, &p)| a == -1 || a as usize == p) {
            println!("Yes");
            println!("{}", p.iter().join(" "));
            return;
        }
    }

    println!("No");
}
