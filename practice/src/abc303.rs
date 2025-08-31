use proconio::{input, marker::Chars};

fn main() {
    input! {
      n: usize,
       s:Chars,
      t:Chars,
    }

    let mut is_match = true;
    for i in 0..n {
        let s_char = s[i];
        let t_char = t[i];

        if s_char != t_char {
            if !((s_char == '1' && t_char == 'l')
                || (s_char == 'l' && t_char == '1')
                || (s_char == 'o' && t_char == '0')
                || (s_char == '0' && t_char == 'o'))
            {
                is_match = false;
                break;
            }
        }
    }

    println!("{}", if is_match { "Yes" } else { "No" })
}
