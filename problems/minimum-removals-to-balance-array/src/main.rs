struct Solution;

impl Solution {
    pub fn min_removal(nums: Vec<i32>, k: i32) -> i32 {
        let mut a: Vec<i64> = nums.into_iter().map(|x| x as i64).collect();
        a.sort_unstable();

        let k = k as i64;
        let n = a.len();
        let mut best = 1usize;
        let mut r = 0usize;

        for l in 0..n {
            if r < l {
                r = l;
            }
            while r < n && a[r] <= a[l] * k {
                r += 1;
            }
            best = best.max(r - l);
        }

        (n - best) as i32
    }
}

fn main() {
    let nums = vec![2, 1, 5];
    let ans = Solution::min_removal(nums.clone(), 2);
    println!("nums = {:?}, k = {}, ans = {}", nums, 2, ans);
    assert_eq!(ans, 1);

    let nums = vec![1, 6, 2, 9];
    let ans = Solution::min_removal(nums.clone(), 3);
    println!("nums = {:?}, k = {}, ans = {}", nums, 3, ans);
    assert_eq!(ans, 2);

    let nums = vec![4, 6];
    let ans = Solution::min_removal(nums.clone(), 2);
    println!("nums = {:?}, k = {}, ans = {}", nums, 2, ans);
    assert_eq!(ans, 0);
}
