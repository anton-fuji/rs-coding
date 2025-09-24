use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
      d: String,
    }

    let res = d
        .chars()
        .map(|c| match c {
            'N' => 'S',
            'S' => 'N',
            'E' => 'W',
            'W' => 'E',
            _ => c,
        })
        .collect::<String>();
    println!("{res}");
}
