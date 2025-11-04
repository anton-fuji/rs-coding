use proconio::{fastout, input};

#[derive(Debug)]
struct UnionFind {
    parent: Vec<isize>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        Self {
            parent: vec![-1; n],
        }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] < 0 {
            x
        } else {
            let root = self.find(self.parent[x] as usize);
            self.parent[x] = root as isize;
            root
        }
    }

    fn unite(&mut self, x: usize, y: usize) -> bool {
        let mut x_root = self.find(x);
        let mut y_root = self.find(y);
        if x_root == y_root {
            return false;
        }

        if self.parent[x_root] > self.parent[y_root] {
            std::mem::swap(&mut x_root, &mut y_root);
        }

        let _ = self.parent[x_root] > self.parent[y_root];
        self.parent[y_root] = x_root as isize;
        true
    }

    fn same(&mut self, x: usize, y: usize) -> bool {
        self.find(x) == self.find(y)
    }
}

#[fastout]
fn main() {
    input! {
      h: usize,
      w: usize,
      q: usize,
    }

    // 0:white  1:red
    let mut grid = vec![vec![0; w]; h];

    let mut uf = UnionFind::new(h * w);

    let dirs = [(1i32, 0), (-1, 0), (0, 1), (0, -1)];

    let id = |r: usize, c: usize| -> usize { r * w + c };

    for _ in 0..q {
        input! {t:usize}

        if t == 1 {
            input! {r: usize, c: usize}
            let r = r - 1;
            let c = c - 1;

            grid[r][c] = 1;
            let cur = id(r, c);

            // 上下左右チェック
            for &(dr, dc) in &dirs {
                let nr = r as i32 + dr;
                let nc = c as i32 + dc;
                if nr < 0 || nr >= h as i32 || nc < 0 || nc >= w as i32 {
                    continue;
                }
                let nr = nr as usize;
                let nc = nc as usize;
                if grid[nr][nc] == 1 {
                    uf.unite(cur, id(nr, nc));
                }
            }
        } else {
            input! {ra: usize, ca: usize, rb: usize, cb: usize}
            let ra = ra - 1;
            let ca = ca - 1;
            let rb = rb - 1;
            let cb = cb - 1;

            if grid[ra][ca] == 1 && grid[rb][cb] == 1 && uf.same(id(ra, ca), id(rb, cb)) {
                println!("Yes");
            } else {
                println!("No");
            }
        }
    }
}
