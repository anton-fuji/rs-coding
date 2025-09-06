use proconio::input;

fn main() {
    input! {
        n: usize,
      a: [i64; n],
      x: i64,
    }

    let mut flg = "No";
    for i in 0..n {
        if x == a[i] {
            flg = "Yes";
            break;
        }
    }
    println!("{flg}");
}
