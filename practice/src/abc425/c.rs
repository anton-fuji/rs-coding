use proconio::input;

fn main() {
    input! {
        n: usize,
      q:usize,
        a: [usize; n],
    }

    let mut offset = 0;
    let mut s = Vec::with_capacity(n + 1);
    s.push(0);
    for i in 0..n {
        s.push(s[i] + a[i]);
    }
    for _ in 0..q {
        input! {q_type:usize}
        match q_type {
            1 => {
                input! {c:usize}
                offset += c;
                offset %= n;
            }
            _ => {
                input! {mut l:usize, mut r:usize}
                let lc = (l - 1 + offset) % n;
                let rc = (r - 1 + offset) % n;
                let res = if rc < lc {
                    s[n] - s[lc] + s[rc + 1]
                } else {
                    s[rc + 1] - s[lc]
                };
                println!("{res}");
            }
        }
    }
}
