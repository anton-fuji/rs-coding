use proconio::{fastout, input, marker::Chars};

#[fastout]
#[allow(unused_imports, non_snake_case)]
fn main() {
    input! {
        a: [[usize; 6]; 3]
    }

    let mut cnt = 0;
    for i in 0..6 {
        for j in 0..6 {
            for k in 0..6 {
                let faces = [a[0][i], a[1][j], a[2][k]];
                let four = faces.contains(&4);
                let five = faces.contains(&5);
                let six = faces.contains(&6);
                if four && five && six {
                    cnt += 1;
                }
            }
        }
    }
    println!("{:.7}", cnt as f64 / 216.0);
}
