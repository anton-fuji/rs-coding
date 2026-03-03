use proconio::{fastout, input, marker::Chars};

#[fastout]
#[allow(unused_imports)]
fn main() {
    input! {
        s: String,
    }

    let mut freq = [0; 26];
    for c in s.chars() {
        freq[(c as u8 - b'a') as usize] += 1;
    }

    let max = *freq.iter().max().unwrap();

    let res: String = s
        .chars()
        .filter(|&c| freq[(c as u8 - b'a') as usize] != max)
        .collect();
    println!("{}", res)
}
