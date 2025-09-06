use itertools::Itertools;
use proconio::input;

fn is_palindrome(v: &String, a: u64, n: u64) -> bool {
    let mut v = v.parse::<u64>().unwrap();
    if v > n {
        return false;
    }
    let mut d = vec![];
    while v > 0 {
        d.push(v % a);
        v /= a;
    }
    (0..d.len()).all(|i| d[i] == d[d.len() - i - 1])
}

fn main() {
    input! {
      a: u64,
       n: u64,
    }
    let mut res = 0;
    for i in 1..=1_000_000 {
        let s = i.to_string();
        let sr = s.chars().rev().join("");
        let v1 = s.clone() + &sr;
        let v2 = s + &sr[1..];

        if is_palindrome(&v1, a, n) {
            res += v1.parse::<u64>().unwrap()
        };

        if is_palindrome(&v2, a, n) {
            res += v2.parse::<u64>().unwrap()
        };
    }
    println!("{res}");
}
