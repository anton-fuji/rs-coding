use proconio::input;

fn main() {
    input! {
       a: [usize; 3]
    }

    let ab = a[0] + a[1];
    let ac = a[0] + a[2];
    let bc = a[1] + a[2];

    let mut res = "No";
    if ab == a[2] {
        res = "Yes";
    } else if ac == a[1] {
        res = "Yes";
    } else if bc == a[0] {
        res = "Yes";
    } else if a[0] == a[1] && a[1] == a[2] {
        res = "Yes";
    }

    println!("{res}");
}
