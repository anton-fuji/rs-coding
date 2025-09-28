use proconio::input;

fn main() {
    input! {
     s: String,
    }

    let s_first = s.chars().next().unwrap();
    let res = format!("{}UPC", s_first);
    println!("{res}");
}
