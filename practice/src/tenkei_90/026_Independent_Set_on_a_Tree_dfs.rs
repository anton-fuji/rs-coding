use itertools::Itertools;
use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input! {
      n: usize,
    }

    let mut graph = vec![Vec::new(); n];
    for _ in 0..(n - 1) {
        input! {a: Usize1, b:Usize1}
        graph[a].push(b);
        graph[b].push(a);
    }

    let mut color = vec![-1; n];
    dfs(0, &graph, &mut color, 0);

    let c_num_0 = color.iter().filter(|&&c| c == 0).count();
    let c_num_1 = color.iter().filter(|&&c| c == 1).count();

    let c = if c_num_0 > c_num_1 { 0 } else { 1 };
    let mut res = Vec::with_capacity(n / 2);
    for i in 0..n {
        if color[i] == c {
            res.push(i + 1);
        }
        if res.len() == (n / 2) {
            break;
        }
    }

    println!("{}", res.iter().format(" "));
}

fn dfs(v: usize, graph: &Vec<Vec<usize>>, color: &mut Vec<i64>, curr: i64) {
    color[v] = curr;
    let next = &graph[v];
    for &u in next {
        if color[u] == -1 {
            dfs(u, graph, color, 1 - curr);
        }
    }
}
