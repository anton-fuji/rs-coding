use std::collections::VecDeque;

use proconio::*;

#[fastout]
fn main() {
    input! {
          n: usize,
    ab: [(usize, usize); n]
      }

    let mut to = vec![vec![]; n];
    let mut got = vec![];
    for (i, &(a, b)) in ab.iter().enumerate() {
        if a == 0 {
            got.push(i);
        } else {
            let a = a - 1;
            let b = b - 1;
            to[a].push(i);
            to[b].push(i);
        }
    }
    let mut visited = vec![false; n];
    let mut q = VecDeque::new();

    for &v in &got {
        visited[v] = true;
        q.push_back(v);
    }
    while let Some(v) = q.pop_front() {
        for &u in &to[v] {
            if visited[u] {
                continue;
            }
            visited[u] = true;
            q.push_back(u);
        }
    }

    let res = visited.iter().filter(|&&b| b).count();
    println!("{res}");
}
