use std::collections::HashSet;

use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
      h: usize,
      _w: usize,
    mut x: usize,
    mut  y: usize,
      s: [Chars; h],
      t: Chars
    }

    x -= 1;
    y -= 1;

    // 行・列
    let mut visited = HashSet::new();
    if s[x][y] == '@' {
        visited.insert((x, y));
    }

    for d in t {
        let (dx, dy) = match d {
            'U' => (-1, 0),
            'D' => (1, 0),
            'L' => (0, -1),
            'R' => (0, 1),
            _ => continue,
        };

        let nx = (x as isize + dx) as usize;
        let ny = (y as isize + dy) as usize;

        if s[nx][ny] != '#' {
            x = nx;
            y = ny;
        }
        if s[nx][ny] == '@' {
            visited.insert((x, y));
        }
    }

    println!("{} {} {}", x + 1, y + 1, visited.len());
}
