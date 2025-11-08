use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
      _n : usize,
      s: String,
    }

    let a_pos: Vec<usize> = s
        .chars()
        .enumerate()
        .filter_map(|(i, c)| if 'A' == c { Some(i) } else { None })
        .collect();

    let res1 = a_pos
        .iter()
        .enumerate()
        .map(|(i, &pos)| pos.abs_diff(2 * i))
        .sum::<usize>();

    let res2 = a_pos
        .iter()
        .enumerate()
        .map(|(i, &pos)| pos.abs_diff(2 * i + 1))
        .sum::<usize>();

    println!("{}", res1.min(res2));
}
