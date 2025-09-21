use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
     q: usize,
    }

    let mut stack = vec![0; 100];
    for _ in 0..q {
        input! {
          q_type: usize,
        }
        match q_type {
            1 => {
                input! {
                  x: i32,
                }
                stack.push(x);
            }
            _ => {
                let last = stack.pop();
                println!("{:?}", last.unwrap());
            }
        }
    }
}
