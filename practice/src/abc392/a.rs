use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
     a: [usize; 3],
    }

    let mut res = "No";
    if a[0] == a[1] * a[2] {
        res = "Yes"
    } else if a[1] == a[0] * a[2] {
        res = "Yes";
    } else if a[2] == a[0] * a[1] {
        res = "Yes";
    }
    println!("{res}");
}
