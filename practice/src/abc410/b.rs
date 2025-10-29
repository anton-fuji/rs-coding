use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
      q: usize,
    }

    let mut count = vec![0; n];
    let mut res = Vec::new();
    for _ in 0..q {
        input! {x:usize}
        if x >= 1 {
            count[x - 1] += 1;
            res.push(x);
        }
        if x == 0 {
            let (i, _) = count
                .iter()
                .enumerate()
                .min_by_key(|&(_i, &val)| val)
                .unwrap();

            count[i] += 1;
            res.push(i + 1);
        }
    }
    println!(
        "{}",
        res.iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join(" ")
    );
}
