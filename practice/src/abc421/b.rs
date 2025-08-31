use proconio::input;

fn reverse(n: u64) -> u64 {
    let mut rev = 0;
    let mut num = n;
    while num > 0 {
        rev = rev * 10 + num % 10;
        num /= 10;
    }
    rev
}

fn main() {
    input! {
      x: u64,
      y: u64,
    }

    let mut ans: u64 = y;
    let mut before = x;
    for _i in 2..10 {
        let tmp = ans;
        ans = reverse(before + tmp);
        before = tmp;
        // println!("{before}:{ans}")
    }
    println!("{ans}");
}
