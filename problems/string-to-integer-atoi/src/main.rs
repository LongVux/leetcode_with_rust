struct Solution;

impl Solution {
    pub fn my_atoi(s: String) -> i32 {
        let bytes = s.as_bytes();
        let mut i = 0usize;
        let n = bytes.len();

        while i < n && bytes[i] == b' ' {
            i += 1;
        }

        let mut sign: i64 = 1;
        if i < n {
            if bytes[i] == b'-' {
                sign = -1;
                i += 1;
            } else if bytes[i] == b'+' {
                i += 1;
            }
        }

        let limit: i64 = if sign == 1 {
            i32::MAX as i64
        } else {
            -(i32::MIN as i64)
        };

        let mut res: i64 = 0;
        while i < n {
            let b = bytes[i];
            if b < b'0' || b > b'9' {
                break;
            }
            let digit = (b - b'0') as i64;
            res = res * 10 + digit;
            if res > limit {
                return if sign == 1 { i32::MAX } else { i32::MIN };
            }
            i += 1;
        }

        (sign * res) as i32
    }
}

fn main() {
    let s = "42".to_string();
    let ans = Solution::my_atoi(s.clone());
    println!("s = {:?}, ans = {}", s, ans);
    assert_eq!(ans, 42);

    let s = "   -42".to_string();
    let ans = Solution::my_atoi(s.clone());
    println!("s = {:?}, ans = {}", s, ans);
    assert_eq!(ans, -42);

    let s = "4193 with words".to_string();
    let ans = Solution::my_atoi(s.clone());
    println!("s = {:?}, ans = {}", s, ans);
    assert_eq!(ans, 4193);

    let s = "words and 987".to_string();
    let ans = Solution::my_atoi(s.clone());
    println!("s = {:?}, ans = {}", s, ans);
    assert_eq!(ans, 0);

    let s = "-91283472332".to_string();
    let ans = Solution::my_atoi(s.clone());
    println!("s = {:?}, ans = {}", s, ans);
    assert_eq!(ans, -2147483648);

    let s = "91283472332".to_string();
    let ans = Solution::my_atoi(s.clone());
    println!("s = {:?}, ans = {}", s, ans);
    assert_eq!(ans, 2147483647);

    let s = "+1".to_string();
    let ans = Solution::my_atoi(s.clone());
    println!("s = {:?}, ans = {}", s, ans);
    assert_eq!(ans, 1);

    let s = "+-12".to_string();
    let ans = Solution::my_atoi(s.clone());
    println!("s = {:?}, ans = {}", s, ans);
    assert_eq!(ans, 0);

    let s = "".to_string();
    let ans = Solution::my_atoi(s.clone());
    println!("s = {:?}, ans = {}", s, ans);
    assert_eq!(ans, 0);

    let s = "00000-42a1234".to_string();
    let ans = Solution::my_atoi(s.clone());
    println!("s = {:?}, ans = {}", s, ans);
    assert_eq!(ans, 0);
}
