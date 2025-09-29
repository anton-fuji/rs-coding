use proconio::input;

fn main() {
    input! {
     a: usize,
     b: usize,
    }

    let sum = a + b;
    let res = sum.pow(2);
    println!("{res}");
}
