use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
        h: usize,
        w: usize,
        c: [Chars; h]
    }

    // 黒ピクセルの範囲を求めていく
    let t = (0..h).find(|&i| c[i].contains(&'#')).unwrap();
    let b = (0..h).rfind(|&i| c[i].contains(&'#')).unwrap();
    let l = (0..w).find(|&j| (0..h).any(|i| c[i][j] == '#')).unwrap();
    let r = (0..w).rfind(|&j| (0..h).any(|i| c[i][j] == '#')).unwrap();

    // 範囲内だけ出力
    for i in t..=b {
        let row = c[i][l..=r].iter().collect::<String>();
        println!("{}", row);
    }
}
