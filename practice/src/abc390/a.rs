use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
     mut a: [usize; 5]
    }

    let mut sorted_a = a.clone();
    sorted_a.sort();
    let mut count = 0;
    for i in 0..4 {
        if a[i] > a[i + 1] {
            a.swap(i, i + 1);
            count += 1;
        }
    }
    if count == 1 && sorted_a == a {
        println!("Yes");
    } else {
        println!("No");
    }
}
