use proconio::{fastout, input, marker::Chars};

#[fastout]
#[allow(unused_imports)]
fn main() {
    input! {
        n: usize,
    }

    let mut c = vec![vec![0i64; n]; n];

    for i in 0..n {
        for j in i + 1..n {
            input! {v: i64}
            c[i][j] = v;
        }
    }

    for a in 0..n {
        for b in a + 1..n {
            for c_idx in b + 1..n {
                if c[a][c_idx] > c[a][b] + c[b][c_idx] {
                    println!("Yes");
                    return;
                }
            }
        }
    }
    println!("No");
}
