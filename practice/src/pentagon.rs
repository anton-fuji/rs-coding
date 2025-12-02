// https://atcoder.jp/contests/adt_easy_20251202_1/tasks/abc333_b
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
      s: String,
      t: String,
    }

    let ss = is_side(&s);
    let tt = is_side(&t);

    if ss == tt {
        println!("Yes");
    } else {
        println!("No");
    }
}

fn is_adjust(p1: char, p2: char) -> bool {
    let (c1, c2) = if p1 < p2 { (p1, p2) } else { (p2, p1) };
    match c1 {
        'A' => c2 == 'B' || c2 == 'E',
        'B' => c2 == 'C',
        'C' => c2 == 'D',
        'D' => c2 == 'E',
        _ => false,
    }
}

fn is_side(s: &str) -> bool {
    let s1 = s.chars().nth(0).unwrap();
    let s2 = s.chars().nth(1).unwrap();

    is_adjust(s1, s2)
}
