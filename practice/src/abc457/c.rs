use proconio::{fastout, input, marker::Chars};

#[fastout]
#[allow(unused_imports, non_snake_case)]
fn main() {
    input! {
        n: usize,
        k: u64,
    }

    let mut a: Vec<Vec<u64>> = Vec::new();
    for _ in 0..n {
        input! {l: usize, row: [u64; l]}
        a.push(row);
    }
    input! {c: [u64; n]}
    let mut remaining = k;
    for i in 0..n {
        let block_size = c[i] * a[i].len() as u64;
        if remaining <= block_size {
            // ここの区間にkがある
            let idx = (remaining - 1) % a[i].len() as u64;
            println!("{}", a[i][idx as usize]);
            return;
        }
        remaining -= block_size;
    }
}
