use proconio::{fastout, input, marker::Chars};

#[fastout]
#[allow(unused_imports)]
fn main() {
    input! {
        n: usize
    }

    let mut count = vec![0usize; n + 1];

    for x in 1..=n {
        if x * x * 2 > n {
            break;
        }
        for y in (x + 1)..=n {
            let sum = x * x + y * y;
            if sum > n {
                break;
            }
            count[sum] += 1;
        }
    }

    let g: Vec<usize> = (1..=n).filter(|&i| count[i] == 1).collect();
    println!("{}", g.len());
    let res = g
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<_>>()
        .join(" ");
    println!("{}", res);
}
