use std::collections::{HashMap, HashSet};

use proconio::{input, marker::Chars};

fn main() {
    input! {
       h: i32,
      _w: i32,
      mut x: i32,
     mut y: i32,
      s: [Chars; h],
     t: Chars
    }

    let mut set: HashSet<(i32, i32)> = HashSet::new();
    let direct = HashMap::from([('U', (-1, 0)), ('D', (1, 0)), ('L', (0, -1)), ('R', (0, 1))]);

    x = x - 1;
    y = y - 1;
    for d in t {
        let (vx, vy) = direct[&d];
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
    println!("{} {} {}", x + 1, y + 1, set.len());
}
