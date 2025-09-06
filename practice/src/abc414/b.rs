use proconio::input;

fn main() {
    input! {
       n: i64,
    }

    let mut res = String::new();
    let mut total_len = 0;
    for _ in 0..n {
        input! {
          c: char,
          l: usize,
        }
        total_len += l;
        if total_len > 100 {
            println!("Too Long");
            return;
        }
        for _ in 0..l {
            res.push(c);
        }
    }
    println!("{res}");
}
