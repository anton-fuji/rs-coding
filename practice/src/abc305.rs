use proconio::input;

fn main() {
    input! {
      n: i64,
    }

    let res = (5..=100).step_by(5).fold(0, |best, curr| {
        if (curr - n).abs() < (best - n).abs() {
            curr
        } else {
            best
        }
    });
    println!("{res}");
}
