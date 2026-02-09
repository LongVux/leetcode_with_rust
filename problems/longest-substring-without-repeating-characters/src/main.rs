use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut last: HashMap<char, usize> = HashMap::new();
        let mut left = 0usize;
        let mut best = 0usize;

        for (right, ch) in s.chars().enumerate() {
            if let Some(&prev) = last.get(&ch) {
                if prev >= left {
                    left = prev + 1;
                }
            }
            last.insert(ch, right);
            let len = right - left + 1;
            if len > best {
                best = len;
            }
        }

        best as i32
    }
}

fn main() {
    let s = "abcabcbb".to_string();
    let ans = Solution::length_of_longest_substring(s.clone());
    println!("s = {:?}, ans = {}", s, ans);
    assert_eq!(ans, 3);

    let s = "bbbbb".to_string();
    let ans = Solution::length_of_longest_substring(s.clone());
    println!("s = {:?}, ans = {}", s, ans);
    assert_eq!(ans, 1);

    let s = "pwwkew".to_string();
    let ans = Solution::length_of_longest_substring(s.clone());
    println!("s = {:?}, ans = {}", s, ans);
    assert_eq!(ans, 3);

    let s = "".to_string();
    let ans = Solution::length_of_longest_substring(s.clone());
    println!("s = {:?}, ans = {}", s, ans);
    assert_eq!(ans, 0);
}
