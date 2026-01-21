use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
      n: usize,
     mut a: [usize; n]
    }

    let mut stack: Vec<(usize, usize)> = Vec::new();
    for x in a {
        if let Some((v, cnt)) = stack.pop() {
            if v == x {
                let new = cnt + 1;
                if new < 4 {
                    stack.push((v, new));
                }
            } else {
                stack.push((v, cnt));
                stack.push((x, 1));
            }
        } else {
            stack.push((x, 1));
        }
    }
    let res: usize = stack.iter().map(|(_, cnt)| cnt).sum();
    println!("{res}");
}
