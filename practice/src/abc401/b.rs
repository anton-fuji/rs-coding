use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
      n: usize,
    s: [String; n],
    }

    let mut logged = false;
    let mut count = 0;

    for op in s {
        match op.as_str() {
            "login" => {
                logged = true;
            }
            "logout" => {
                logged = false;
            }
            "public" => {}
            "private" => {
                if !logged {
                    count += 1;
                }
            }
            _ => {}
        }
    }
    println!("{count}");
}
