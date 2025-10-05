use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
      x: String,
      y: String,
    }

    let os = ["Ocelot", "Serval", "Lynx"];
    let xi = os.iter().position(|s| s == &x).unwrap();
    let yi = os.iter().position(|s| s == &y).unwrap();

    println!("{}", if yi <= xi { "Yes" } else { "No" });
}
