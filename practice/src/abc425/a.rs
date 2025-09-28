use proconio::*;

#[fastout]
fn main() {
    input! {
        n: u32,
    }

    let a: i32 = -1;
    let mut sum = 0;
    for i in 1..=n {
        let aa = a.pow(i);
        let ai = i.pow(3);
        sum = sum + (aa * ai as i32);
    }
    println!("{sum}");
}
