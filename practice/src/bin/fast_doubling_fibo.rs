use std::time::Instant;

use num_bigint::BigInt;
use num_traits::{One, Zero};

fn fib(n: u64) -> (BigInt, BigInt) {
    if n == 0 {
        return (BigInt::zero(), BigInt::one());
    }

    let (a, b) = fib(n / 2);
    let c = &a * (&b * 2u32 - &a);
    let d = &a * &a + &b * &b;

    if n % 2 == 0 {
        (c, d)
    } else {
        (d.clone(), c + d)
    }
}

fn main() {
    let start = Instant::now();
    let n = 1_000_000u64;
    let (f, _) = fib(n);
    let elapsed = start.elapsed();
    println!("{f}");
    println!("{:?}", elapsed);
}
