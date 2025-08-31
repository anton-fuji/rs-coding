use proconio::input;

fn main() {
    input! {
      s : String,
    }

    let t_idx: Vec<usize> = s
        .chars()
        .enumerate()
        .filter_map(|(i, c)| if c == 't' { Some(i) } else { None })
        .collect();

    let mut t_rate_max = 0.0;
    for i in 0..t_idx.len() {
        for j in i + 1..t_idx.len() {
            let cnt = (j - i + 1) as f64;
            let sub_len = (t_idx[j] - t_idx[i] + 1) as f64;
            let t_rate = (cnt - 2.0) / (sub_len - 2.0);
            t_rate_max = t_rate.max(t_rate_max);
        }
    }
    println!("{t_rate_max}");
}
