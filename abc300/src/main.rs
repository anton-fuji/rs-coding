use proconio::input;

fn main() {
    input! {
      h: usize,
      w: usize,
      a: [String; h],
      b: [String; h],
    }

    let a: Vec<Vec<char>> = a.into_iter().map(|row| row.chars().collect()).collect();
    let b: Vec<Vec<char>> = b.into_iter().map(|row| row.chars().collect()).collect();

    for s in 0..h {
        for t in 0..w {
            if shifted(&a, &b, s, t, h, w) {
                println!("Yes");
                return;
            }
        }
    }
    println!("No")
}

fn shifted(a: &[Vec<char>], b: &[Vec<char>], s: usize, t: usize, h: usize, w: usize) -> bool {
    for i in 0..h {
        for j in 0..w {
            let ni = (i + s) % h;
            let nj = (j + t) % w;
            if a[ni][nj] != b[i][j] {
                return false;
            }
        }
    }
    true
}
