use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {s:[String; 12]}
    let mut cnt = 0;
    for i in 0..12 {
        let s_len = s[i].len();
        if s_len == i + 1 {
            cnt += 1;
        }
    }
    println!("{cnt}");
}
