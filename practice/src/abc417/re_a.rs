use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
      n: usize,
      m: usize,
    mut  a: [usize; n],
      b: [usize; m],
    }


    for x in b {
      if let Some(pos) = a.iter().position(|&v| v == x){
        a.remove(pos);
      }
    }

      if !a.is_empty(){
        for (i, v) in a.iter().enumerate() {
          if i > 0 {
            print!(" ");
          }
          print!("{v}");
      }
    }
    println!()
}
