use proconio::input;

fn sum_digits(n: usize) -> usize {
    let mut num = n;
    let mut ans = 0;
    while num >= 1 {
        ans += num % 10;
        num /= 10;
    }
    ans
}

fn solve(n: usize, a: usize, b: usize) -> usize {
    (1..=n)
        .into_iter()
        .map(|n| (n, sum_digits(n)))
        .filter(|t| t.1 >= a && t.1 <= b)
        .map(|t| t.0)
        .sum()
}

fn main() {
    input! {
      n: usize,
      a: usize,
      b: usize,
    }

    println!("{}", solve(n, a, b));
}
