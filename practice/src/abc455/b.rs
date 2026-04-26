use proconio::{fastout, input, marker::Chars};

#[fastout]
#[allow(unused_imports, non_snake_case)]
fn main() {
    input! {
        h: usize,
        w: usize,
        s :[Chars; h]
    }

    let mut res = 0;
    for h1 in 0..h {
        for h2 in h1..h {
            for w1 in 0..w {
                for w2 in w1..w {
                    let mut ok = true;
                    for i in h1..=h2 {
                        for j in w1..=w2 {
                            let si = h1 + h2 - i;
                            let sj = w1 + w2 - j;
                            if s[i][j] != s[si][sj] {
                                ok = false;
                                break;
                            }
                        }
                    }
                    if ok {
                        res += 1
                    }
                }
            }
        }
    }
    println!("{}", res);
}
