use std::{cmp::Reverse, collections::BinaryHeap};

use proconio::{fastout, input, marker::Chars};

#[fastout]
#[allow(unused_imports, non_snake_case)]
fn main() {
    input! {
        q: usize
    }

    // BinaryHeapを使うとクエリ2で「h以下の木を全削除」するとき、
    // 木の高さが小さい順に取り出せると効率的
    // 小さい順 = 最小ヒープが最適
    let mut hp: BinaryHeap<Reverse<i64>> = BinaryHeap::new();
    for _ in 0..q {
        input! {t: usize, h: i64}
        if t == 1 {
            hp.push(Reverse(h));
        } else {
            while let Some(&Reverse(top)) = hp.peek() {
                if top <= h {
                    hp.pop();
                } else {
                    break;
                }
            }
        }
        println!("{}", hp.len());
    }
}
