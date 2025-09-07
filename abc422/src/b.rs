use std::isize;

use proconio::{input, marker::Chars};

fn main() {
    input! {
      h: usize,
      w: usize,
      grid: [Chars; h],
    }

    let dx = [0, 0, 1, -1];
    let dy = [1, -1, 0, 0];

    for i in 0..h {
        for j in 0..w {
            if grid[i][j] == '#' {
                let mut ok_block = 0;
                for k in 0..4 {
                    let ni = (i as isize) + dx[k];
                    let nj = (j as isize) + dy[k];

                    if ni >= 0 && ni < h as isize && nj >= 0 && nj < w as isize {
                        let ni = ni as usize;
                        let nj = nj as usize;
                        if grid[ni][nj] == '#' {
                            ok_block += 1;
                        }
                    }
                }
                if ok_block != 2 && ok_block != 4 {
                    println!("No");
                    return;
                }
            }
        }
    }
    println!("Yes");
}
