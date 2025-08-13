use proconio::input;

fn main() {
    input! {
      n: usize,
      s: [String; n],
    }

    let mut is_logged_in = false;
    let mut count = 0;

    for op in s {
        // as_str():String型の所有権を移動させずに&strとして参照できる
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
            // 上記にマッチしなかった全てのケースに対応させるため
            _ => {}
        }
    }
    println!("{count}")
}
