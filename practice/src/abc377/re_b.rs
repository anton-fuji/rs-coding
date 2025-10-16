use proconio::{fastout, input, marker::Chars};

const N: usize = 8;
#[fastout]
fn main() {
    input! {
     s: [Chars; N]
    }

    let rows = s.iter().filter(|row| !row.contains(&'#')).count();
    let col = (0..N)
        .filter(|&i| s.iter().all(|row| row[i] != '#'))
        .count();

    println!("{}", rows * col);
}
