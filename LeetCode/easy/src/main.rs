struct Solution;

impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        match haystack.find(&needle) {
            Some(idx) => idx as i32,
            None => -1,
        }
    }
}

fn main() {
    let haystack = "leetcode".to_string();
    let needle = "leeto".to_string();

    println!("ans:{}", Solution::str_str(haystack, needle));
}
