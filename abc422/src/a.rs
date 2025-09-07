use proconio::input;

fn main() {
    input! {
      s: String,
    }

    let stage: Vec<i32> = s.split('-').map(|s| s.parse().unwrap()).collect();

    let mut i = stage[0];
    let mut j = stage[1];

    if j < 8 {
        j += 1;
    } else {
        i += 1;
        j = 1
    }

    println!("{i}-{j}");
}
