use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
     s: String,
    }

    let ss: Vec<char> = s.chars().collect();
    let s_len = s.len();
    let mut count = 0;

    for j in 1..s_len - 1 {
        if ss[j] == 'B' {
            for d in 1..=s_len {
                let i = j as isize - d as isize;
                let k = j as isize + d as isize;

                if i >= 0 && k < s_len as isize {
                    if ss[i as usize] == 'A' && ss[k as usize] == 'C' {
                        count += 1;
                    }
                } else {
                    break;
                }
            }
        }
    }
    println!("{count}");
}
