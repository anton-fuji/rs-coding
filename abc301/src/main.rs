use proconio::input;

fn main() {
    input! {
      n :usize,
      s: String,
    }

    let mut cnt_t = 0;
    let mut cnt_a = 0;
    for c in s.chars() {
        if c == 'T' {
            cnt_t += 1;
        } else {
            cnt_a += 1;
        }
    }

    if cnt_t > cnt_a {
        println!("T");
    } else if cnt_t < cnt_a {
        println!("A");
    } else {
        let m = cnt_t;
        let mut reach_t: Option<usize> = None;
        let mut reach_a: Option<usize> = None;
        let mut curr_t = 0;
        let mut curr_a = 0;

        for (i, c) in s.chars().enumerate() {
            if c == 'T' {
                curr_t += 1;
                if curr_t == m && reach_t.is_none() {
                    reach_t = Some(i);
                }
            } else {
                curr_a += 1;
                if curr_a == m && reach_a.is_none() {
                    reach_a = Some(i);
                }
            }
        }
        if reach_t.unwrap() < reach_a.unwrap() {
            println!("T");
        } else {
            println!("A");
        }
    }
}
