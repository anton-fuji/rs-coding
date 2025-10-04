use proconio::input;

fn main() {
    input! {
      n:usize,
     mut r:isize,
    }

    for _ in 0..n {
        input! {
          d: usize,
          a: isize
        }
        if a <= 2400 {
            match d {
                1 => {
                    if 1600 <= r && r <= 2799 {
                        r += a;
                    }
                }
                _ => {
                    if 1200 <= r && r <= 2399 {
                        r += a
                    }
                }
            }
        }
    }
    println!("{r}");
}
