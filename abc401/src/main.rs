use proconio::input;

fn main() {
    input! {
      n: usize,
      s: [String; n],
    }

    let mut is_logged_in = false;
    let mut count = 0;

    for op in s {
        match op.as_str() {
            "login" => {
                is_logged_in = true;
            }
            "logout" => {
                is_logged_in = false;
            }
            "public" => {}
            "private" => {
                if !is_logged_in {
                    count += 1;
                }
            }
            _ => {}
        }
    }
    println!("{count}")
}
