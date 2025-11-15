use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
      a: i32,
      b: i32,
      c: i32,
    }

    let v = [a, b, c];
    let mut patterns = Vec::new();

    for i in 0..3 {
        for j in 0..3 {
            if j == i {
                continue;
            }
            for k in 0..3 {
                if k == i || k == j {
                    continue;
                }
                let num = v[i] * 100 + v[j] * 10 + v[k];
                patterns.push(num);
            }
        }
    }
    let res = patterns.iter().max().unwrap();
    println!("{res}");
}
