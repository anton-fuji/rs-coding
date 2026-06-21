use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
        n: usize,
    }

    let mut res: Vec<Vec<usize>> = vec![Vec::new(); n + 1];
    for i in 1..=n {
        input! {
            k: i64,
            a: [usize; k]
        }
        for &to in &a {
            res[to].push(i);
        }
    }
    for i in 1..=n {
        let list = &res[i];
        print!("{} ", list.len());
        for &p in list {
            print!("{} ", p);
        }
        println!();
    }
}

