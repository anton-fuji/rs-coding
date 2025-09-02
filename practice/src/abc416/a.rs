use proconio::input;

fn main() {
    input! {
      _: usize,
      l: usize,
      r: usize,
      s: String,
    }

    let s_char: Vec<char> = s.chars().collect();
    let mut flg = true;
    for &i in s_char.iter().take(r).skip(l - 1) {
        if i != 'o' {
            flg = false;
            break;
        }
    }

    if flg {
        println!("Yes");
    } else {
        println!("No");
    }
}
/*   よりシンプル
    if s_char.iter().take(r).skip(l - 1).all(|&c| c == 'o') {
        println!("Yes");
    } else {
        println!("No");
    }
*/
