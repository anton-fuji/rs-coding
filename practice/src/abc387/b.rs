use proconio::input;

fn main() {
    input! {
     x: usize,
    }

    let mut sum = 0;
    for i in 1..=9 {
        for j in 1..=9 {
            if (i * j) != x {
                sum += i * j;
            }
        }
    }
    println!("{sum}");
}
