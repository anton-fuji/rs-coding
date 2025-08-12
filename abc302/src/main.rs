use proconio::input;

fn main() {
    input! {
      a: usize,
      b: usize,
    }

    // let attack = (a + b - 1) / b;
    let attack = a.div_ceil(b);

    println!("{attack}")
}
