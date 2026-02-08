struct Solution;

impl Solution {
    pub fn minimum_cost(nums: Vec<i32>) -> i32 {
        let first = nums[0];
        let mut min1 = i32::MAX;
        let mut min2 = i32::MAX;
        for &x in nums.iter().skip(1) {
            if x < min1 {
                min2 = min1;
                min1 = x;
            } else if x < min2 {
                min2 = x;
            }
        }
        first + min1 + min2
    }
}

fn main() {
    let nums = vec![1, 2, 3, 12];
    let ans = Solution::minimum_cost(nums.clone());
    println!("nums = {:?}, ans = {}", nums, ans);
    assert_eq!(ans, 6);

    let nums = vec![5, 4, 3];
    let ans = Solution::minimum_cost(nums.clone());
    println!("nums = {:?}, ans = {}", nums, ans);
    assert_eq!(ans, 12);

    let nums = vec![10, 3, 1, 1];
    let ans = Solution::minimum_cost(nums.clone());
    println!("nums = {:?}, ans = {}", nums, ans);
    assert_eq!(ans, 12);
}
