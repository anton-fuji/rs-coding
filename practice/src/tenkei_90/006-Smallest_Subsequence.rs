use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
      n:usize,
      mut k: usize,
      s: Chars,
    }

    let mut cnt = n - k;
    let mut stack = vec![];
    for c in s {
        while let Some(&top) = stack.last() {
            if cnt > 0 && top > c {
                stack.pop();
                cnt -= 1;
            } else {
                break;
            }
        }
        stack.push(c);
    }

    for _ in 0..cnt {
        stack.pop();
    }

    let res = stack.iter().collect::<String>();
    println!("{res}");
}
