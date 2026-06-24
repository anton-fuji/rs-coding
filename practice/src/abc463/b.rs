use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
        n: usize,
        x: char,
    }

    let m = match x {
        'A' => 0,
        'B' => 1,
        'C' => 2,
        'D' => 3,
        'E' => 4,
        _ => unreachable!(),
    };

    for i in 0..n {
        input! {s: Chars}
        for j in 0..5 {
            if j == m && s[j] == 'o' {
                println!("Yes");
                return;
            }
        }
    }
    println!("No");
}
