use proconio::{fastout, input, marker::Chars};

#[fastout]
#[allow(unused_imports, non_snake_case)]
fn main() {
    input! {
        n:usize,
        s: [String; n],
    }

    let mut res = String::new();
    for si in s.iter() {
        let c = match si.chars().next().unwrap() {
            'a'..='c' => '2',
            'd'..='f' => '3',
            'g'..='i' => '4',
            'j'..='l' => '5',
            'm'..='o' => '6',
            'p'..='s' => '7',
            't'..='v' => '8',
            'w'..='z' => '9',
            _ => unreachable!(),
        };
        res.push(c);
    }
    println!("{}", res);
}
