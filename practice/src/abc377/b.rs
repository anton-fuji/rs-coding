use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
     s: [Chars; 8]
    }

    let mut count = 0;

    for i in 0..8 {
        for j in 0..8 {
            let mut ok = true;
            for k in 0..8 {
                if s[i][k] == '#' || s[k][j] == '#' {
                    ok = false;
                }
            }
            if ok {
                count += 1;
            }
        }
    }
    println!("{count}");
}
