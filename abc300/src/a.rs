use proconio::input;

fn main() {
    input! {
      n: usize,
      a: i32,
      b: i32,
      c: [i32; n],
    }

    let sum = a + b;
    for (i, &v) in c.iter().enumerate() {
        if v == sum {
            println!("{}", i + 1);
            break;
        }
    }
}
