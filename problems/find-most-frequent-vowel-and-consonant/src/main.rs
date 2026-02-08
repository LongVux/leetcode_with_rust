struct Solution;

impl Solution {
    pub fn most_frequent_vowel_and_consonant(s: &str) -> i32 {
        let mut counts = [0i32; 26];

        for ch in s.chars() {
            let c = ch.to_ascii_lowercase();
            if !c.is_ascii_alphabetic() {
                continue;
            }
            let idx = (c as u8 - b'a') as usize;
            counts[idx] += 1;
        }

        let vowels = [
            true, false, false, false, true, false, false, false, true, false, false, false, false,
            false, true, false, false, false, false, false, true, false, false, false, false,
            false,
        ];

        let mut best_vowel = ('?', 0);
        let mut best_cons = ('?', 0);

        for i in 0..26 {
            let c = (b'a' + i as u8) as char;
            let cnt = counts[i];
            if vowels[i] {
                if cnt > best_vowel.1 || (cnt == best_vowel.1 && cnt > 0 && c < best_vowel.0) {
                    best_vowel = (c, cnt);
                }
            } else {
                if cnt > best_cons.1 || (cnt == best_cons.1 && cnt > 0 && c < best_cons.0) {
                    best_cons = (c, cnt);
                }
            }
        }

        best_vowel.1 + best_cons.1
    }
}

fn main() {
    let s = "Hello, World!";
    let sum = Solution::most_frequent_vowel_and_consonant(s);
    println!("s = {:?}", sum);

    let s = "leetcode";
    let sum2 = Solution::most_frequent_vowel_and_consonant(s);
    println!("s = {:?}", sum2);
}
