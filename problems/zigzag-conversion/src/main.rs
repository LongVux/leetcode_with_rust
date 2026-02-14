struct Solution;

impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        let num_rows = num_rows as usize;
        if num_rows <= 1 {
            return s;
        }

        let mut rows = vec![String::new(); num_rows];
        let mut row: i32 = 0;
        let mut dir: i32 = 1;

        for ch in s.chars() {
            rows[row as usize].push(ch);
            if row == 0 {
                dir = 1;
            } else if row == num_rows as i32 - 1 {
                dir = -1;
            }
            row += dir;
        }

        rows.concat()
    }
}

fn main() {
    let s = "PAYPALISHIRING".to_string();
    let ans = Solution::convert(s.clone(), 3);
    println!("s = {:?}, num_rows = 3, ans = {:?}", s, ans);
    assert_eq!(ans, "PAHNAPLSIIGYIR");

    let s = "PAYPALISHIRING".to_string();
    let ans = Solution::convert(s.clone(), 4);
    println!("s = {:?}, num_rows = 4, ans = {:?}", s, ans);
    assert_eq!(ans, "PINALSIGYAHRPI");

    let s = "A".to_string();
    let ans = Solution::convert(s.clone(), 1);
    println!("s = {:?}, num_rows = 1, ans = {:?}", s, ans);
    assert_eq!(ans, "A");

    let s = "AB".to_string();
    let ans = Solution::convert(s.clone(), 3);
    println!("s = {:?}, num_rows = 3, ans = {:?}", s, ans);
    assert_eq!(ans, "AB");

    let s = "ABCDEF".to_string();
    let ans = Solution::convert(s.clone(), 2);
    println!("s = {:?}, num_rows = 2, ans = {:?}", s, ans);
    assert_eq!(ans, "ACEBDF");
}
