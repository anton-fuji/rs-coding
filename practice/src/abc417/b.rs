use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        mut a: [i32; n],
        mut b: [i32; m],
    }
    a.sort();
    b.sort();

    let mut res = Vec::new();
    let mut b_iter = b.iter().peekable();

    for &a_val in a.iter() {
        // b_iterの現在要素がa_valより小さい間は、b_iterを進めます。
        while let Some(&&b_val) = b_iter.peek() {
            if b_val < a_val {
                b_iter.next();
            } else {
                break;
            }
        }

        // b_iterの現在要素がa_valと等しい場合、その要素はbに含まれているのでスキップ
        if let Some(&&b_val) = b_iter.peek() {
            if b_val == a_val {
                // bにもa_valがあるので、b_iterも進める
                b_iter.next();
                continue;
            }
        }
        // b_iterの現在要素がa_valと等しくない場合、その要素はaにしか含まれていないので結果に追加
        res.push(a_val);
    }
    println!("{}", res.iter().join(" "));
}
