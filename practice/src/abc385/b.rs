use proconio::{fastout, input, marker::Chars};
use std::collections::{HashMap, HashSet};

#[fastout]
fn main() {
    input! {
      h: i32,
      _w: i32,
    mut x: i32,
    mut  y: i32,
      s: [Chars; h],
      t: Chars
    }

    // 行・列
    let drct = HashMap::from([('U', (-1, 0)), ('D', (1, 0)), ('L', (0, -1)), ('R', (0, 1))]);
    let mut set: HashSet<(i32, i32)> = HashSet::new();

    x -= 1;
    y -= 1;
    for d in t {
        let (vx, vy) = drct[&d];
        let nx = x + vx;
        let ny = y + vy;
        if s[nx as usize][ny as usize] != '#' {
            x = nx;
            y = ny;
        }
        if s[x as usize][y as usize] == '@' {
            set.insert((x, y));
        }
    }
    println!("{} {} {}", x + 1, y + 1, set.len())
}
