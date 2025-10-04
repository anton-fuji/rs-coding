use proconio::input;

fn main() {
    input! {
      n :usize
    }

    let mut v: Vec<String> = Vec::new();
    for bit in 0..(1 << n) {
        let mut s = String::new();
        let mut ret = 0;
        for i in 0..n {
            if bit & (1 << i) != 0 {
                s.insert(0, ')');
                ret += 1;
            } else {
                s.insert(0, '(');
                ret -= 1;
            }
            if ret == -1 {
                break;
            }
        }
        if ret == 0 {
            v.push(s);
        }
    }
    for s in v {
        println!("{s}");
    }
}
