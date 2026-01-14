use std::collections::VecDeque;

use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
      n: usize,
      m: usize,
      e :[(usize, usize); m]
    }

    if m != n {
        println!("No");
        return;
    }

    let mut deg = vec![0; n];
    let mut g = vec![vec![]; n];

    for (a, b) in e {
        let a = a - 1;
        let b = b - 1;

        deg[a] += 1;
        deg[b] += 1;
        g[a].push(b);
        g[b].push(a);
    }

    // 次数が2かのチェック
    if deg.iter().any(|&d| d != 2) {
        println!("No");
        return;
    }

    // BFS
    let mut visited = vec![false; n];
    let mut q = VecDeque::new();
    visited[0] = true;
    q.push_back(0);

    while let Some(v) = q.pop_front() {
        for &u in &g[v] {
            if !visited[u] {
                visited[u] = true;
                q.push_back(u);
            }
        }
    }

    if visited.iter().all(|&v| v) {
        println!("Yes")
    } else {
        println!("No")
    }
}
