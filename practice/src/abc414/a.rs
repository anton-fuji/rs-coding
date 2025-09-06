use proconio::input;

fn main() {
    input! {
       n: i64,
      l: i64,
      r: i64,
    }

    let mut count = 0;
    for _ in 0..n {
        input! {x: i64, y: i64}
        if l >= x && r <= y {
            count += 1;
        }
    }

    println!("{count}");
}
