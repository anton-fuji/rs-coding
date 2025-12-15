use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {s:Chars}
    let mut pos = vec![0; 26];
    for i in 0..26 {
        let char = s[i] as usize - 'A' as usize;
        pos[char] = i;
    }
    let mut total_dist = 0;
    for i in 0..25 {
        total_dist += pos[i].abs_diff(pos[i + 1]);
    }
    println!("{total_dist}");
}
