use proconio::input;

fn main() {
    input! {
      n: u64,
      m : u64,
    }

    let limit = 1_000_000_000u64;
    let mut sum: u64 = 0;

    let mut curr_n: u64 = 1;
    for i in 0..=m {
        if sum > limit {
            println!("inf");
            return;
        }

        if i > 0 {
            if let Some(next_power) = curr_n.checked_mul(n) {
                curr_n = next_power;
            } else {
                println!("inf");
                return;
            }
        }
        if let Some(next_sum) = sum.checked_add(curr_n) {
            sum = next_sum;
        } else {
            println!("inf");
            return;
        }
    }

    if sum > limit {
        println!("inf");
    } else {
        println!("{sum}")
    }
}
