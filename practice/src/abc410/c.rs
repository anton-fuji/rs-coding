use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input! {
      (n, q): (usize, usize)
    }

    let mut a: Vec<usize> = (1..=n).collect();
    let mut offset = 0;
    for _ in 0..q {
        input! {(q_type,pk): (usize,Usize1)}
        match q_type {
            1 => {
                input! {x:usize}
                a[(pk + offset) % n] = x;
            }
            2 => {
                println!("{}", a[(pk + offset) % n]);
            }
            3 => {
                let k = (pk + 1) % n;
                offset = (offset + k) % n;
            }
            _ => unreachable!(),
        }
    }
}
