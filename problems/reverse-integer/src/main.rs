struct Solution;

impl Solution {
    pub fn reverse(x: i32) -> i32 {
        let mut x = x as i64;
        let mut rev: i64 = 0;
        let min = i32::MIN as i64;
        let max = i32::MAX as i64;

        while x != 0 {
            let pop = x % 10;
            x /= 10;
            rev = rev * 10 + pop;
            if rev < min || rev > max {
                return 0;
            }
        }

        rev as i32
    }
}

fn main() {
    let x = 123;
    let ans = Solution::reverse(x);
    println!("x = {}, ans = {}", x, ans);
    assert_eq!(ans, 321);

    let x = -123;
    let ans = Solution::reverse(x);
    println!("x = {}, ans = {}", x, ans);
    assert_eq!(ans, -321);

    let x = 120;
    let ans = Solution::reverse(x);
    println!("x = {}, ans = {}", x, ans);
    assert_eq!(ans, 21);

    let x = 0;
    let ans = Solution::reverse(x);
    println!("x = {}, ans = {}", x, ans);
    assert_eq!(ans, 0);

    let x = 1534236469;
    let ans = Solution::reverse(x);
    println!("x = {}, ans = {}", x, ans);
    assert_eq!(ans, 0);

    let x = -2147483648;
    let ans = Solution::reverse(x);
    println!("x = {}, ans = {}", x, ans);
    assert_eq!(ans, 0);
}
