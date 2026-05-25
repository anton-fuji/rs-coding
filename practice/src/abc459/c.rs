use proconio::{fastout, input, marker::Chars};

#[fastout]
#[allow(unused_imports, non_snake_case)]
fn main() {
    input! {
        n:usize,
        q: usize
    }

    let mut blocks = vec![0i64; n + 1];
    let mut offset = 0i64;
    let mut min_b = 0i64;

    for _ in 0..q {
        input! {t: usize, v: i64}
        if t == 1 {
            blocks[v as usize] += 1;
            if blocks[v as usize] < min_b + 1 {
                // まだ全マスが1個以上でない
            } else {
                min_b = *blocks.iter().skip(1).min().unwrap();
                if min_b > offset {
                    offset += 1;
                }
            }
        } else {
            // y 個以上 = 積んだ回数がoffset +v 以上マスの個数
            let y = offset + v;
            let cnt = blocks.iter().skip(1).filter(|&&b| b >= y).count();
            println!("{cnt}");
        }
    }
}
