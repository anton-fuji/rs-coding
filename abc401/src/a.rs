use proconio::input;

fn main() {
    input! {
      s: usize,
    }

    if s >= 200 && s < 300 {
        println!("Success");
    } else {
        println!("Failure")
    }
}
