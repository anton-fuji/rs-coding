use proconio::input;

fn main() {
    input! {
      n: usize,
        sa: [(String, u32); n],
    }

    let (i, _) = sa
        .iter()
        .enumerate()
        .min_by_key(|&(_, &(_, age))| age)
        .unwrap();
    for j in 0..n {
        println!("{}", sa[(j + i) % n].0);
    }
}
