use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
    }

    let mut res = vec![vec![0usize; n]; n];

    let mut r: usize = 0;
    let mut c: usize = (n - 1) / 2;
    let mut k: usize = 1;

    res[r][c] = k;
    k += 1;

    for _ in 0..(n * n - 1) {
        let prev_r = r;
        let prev_c = c;
        r = (r + n - 1) % n;
        c = (c + 1) % n;

        if res[r][c] == 0 {
            res[r][c] = k;
        } else {
            r = (prev_r + 1) % n;
            c = prev_c;
            res[r][c] = k;
        }

        k += 1;
    }

    for row in res {
        println!(
            "{}",
            row.iter()
                .map(|v| v.to_string())
                .collect::<Vec<_>>()
                .join(" ")
        );
    }
}
