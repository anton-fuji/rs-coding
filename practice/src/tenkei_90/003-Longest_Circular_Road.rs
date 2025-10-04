use proconio::{fastout, input};
use std::vec;

#[fastout]
fn main() {
    input! {
      n :usize,
    }

    let mut G = vec![vec![]; n];
    for i in 0..n - 1 {
        input! {mut a: usize, mut b:usize}
        a -= 1;
        b -= 1;
        G[a].push(b);
        G[b].push(a);
    }
    let mut dist = vec![-1; n];
    dist[0] = 0;
    dfs(0, &mut dist, &G);
    let mut max = 0;

    for i in 0..n {
        if dist[i] > dist[max] {
            max = i;
        }
    }
    let mut dist2 = vec![-1; n];
    dist2[max] = 0;
    dfs(max, &mut dist2, &G);
    let res = dist2.iter().max().unwrap() + 1;
    println!("{res}");
}

fn dfs(v: usize, dist: &mut Vec<isize>, G: &Vec<Vec<usize>>) {
    for &u in &G[v] {
        if dist[u] == -1 {
            dist[u] = dist[v] + 1;
            dfs(u, dist, G);
        }
    }
}
