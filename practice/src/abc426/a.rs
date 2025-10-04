use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
     x: String,
      y: String
    }

    let mut res = "No";
    if x == "Ocelot" {
        if y == "Ocelot" {
            res = "Yes";
        }
    }
    if x == "Serval" {
        if y == "Ocelot" {
            res = "Yes";
        }
        if y == "Serval" {
            res = "Yes";
        }
    }
    if x == "Lynx" {
        if y == "Ocelot" {
            res = "Yes";
        }
        if y == "Serval" {
            res = "Yes";
        }
        if y == "Lynx" {
            res = "Yes";
        }
    }
    println!("{res}");
}
