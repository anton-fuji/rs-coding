use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
      t: String,
      u: String,
    }

    let n = t.len();
    let m = u.len();

    let mut possible = false;

    for i in 0..=(n - m) {
        let mut found = true;
        for j in 0..m {
            let t_char = t.chars().nth(i + j).unwrap();
            let u_char = u.chars().nth(j).unwrap();

            if t_char != '?' && t_char != u_char {
                found = false;
                break;
            }
        }
        if found {
            possible = true;
            break;
        }
    }

    if possible {
        println!("Yes");
    } else {
        println!("No");
    }
}
