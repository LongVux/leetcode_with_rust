struct Solution;

impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        let chars: Vec<char> = s.chars().collect();
        let n = chars.len();
        if n <= 1 {
            return s;
        }

        let mut best_l = 0usize;
        let mut best_r = 0usize; // inclusive

        for center in 0..n {
            let (l1, r1) = expand(&chars, center, center);
            if r1 - l1 > best_r - best_l {
                best_l = l1;
                best_r = r1;
            }

            if center + 1 < n && chars[center] == chars[center + 1] {
                let (l2, r2) = expand(&chars, center, center + 1);
                if r2 - l2 > best_r - best_l {
                    best_l = l2;
                    best_r = r2;
                }
            }
        }

        chars[best_l..=best_r].iter().collect()
    }
}

fn expand(chars: &[char], mut l: usize, mut r: usize) -> (usize, usize) {
    let n = chars.len();
    while l > 0 && r + 1 < n && chars[l - 1] == chars[r + 1] {
        l -= 1;
        r += 1;
    }
    (l, r)
}

fn main() {
    let s = "babad".to_string();
    let ans = Solution::longest_palindrome(s.clone());
    println!("s = {:?}, ans = {:?}", s, ans);
    assert!(ans == "bab" || ans == "aba");

    let s = "cbbd".to_string();
    let ans = Solution::longest_palindrome(s.clone());
    println!("s = {:?}, ans = {:?}", s, ans);
    assert_eq!(ans, "bb");

    let s = "a".to_string();
    let ans = Solution::longest_palindrome(s.clone());
    println!("s = {:?}, ans = {:?}", s, ans);
    assert_eq!(ans, "a");

    let s = "ac".to_string();
    let ans = Solution::longest_palindrome(s.clone());
    println!("s = {:?}, ans = {:?}", s, ans);
    assert!(ans == "a" || ans == "c");
}
