use proconio::input;

fn main() {
    input! {
      a:usize,
      b:usize,
      c:usize,
      d:usize,
      e:usize,
    }

    let keys = ['A', 'B', 'C', 'D', 'E'];
    let pair = [a, b, c, d, e];

    let mut pairs = vec![];

    for bit in 1..(1 << 5) {
        let mut name = String::new();
        let mut score = 0;
        for i in 0..5 {
            if bit & (1 << i) != 0 {
                name.push(keys[i]);
                score += pair[i];
            }
        }
        pairs.push((name, score));
    }

    pairs.sort_by(|a, b| {
        if a.1 == b.1 {
            a.0.cmp(&b.0)
        } else {
            b.1.cmp(&a.1)
        }
    });

    for (name, _) in pairs {
        println!("{name}");
    }
}
