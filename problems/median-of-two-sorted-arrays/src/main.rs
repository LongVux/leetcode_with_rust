struct Solution;

impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let (a, b) = if nums1.len() <= nums2.len() {
            (nums1, nums2)
        } else {
            (nums2, nums1)
        };
        let m = a.len();
        let n = b.len();
        let half = (m + n + 1) / 2;

        let mut lo = 0usize;
        let mut hi = m;

        while lo <= hi {
            let i = (lo + hi) / 2; // cut in a
            let j = half - i;      // cut in b

            let a_left = if i == 0 { i32::MIN } else { a[i - 1] };
            let a_right = if i == m { i32::MAX } else { a[i] };
            let b_left = if j == 0 { i32::MIN } else { b[j - 1] };
            let b_right = if j == n { i32::MAX } else { b[j] };

            if a_left <= b_right && b_left <= a_right {
                if (m + n) % 2 == 1 {
                    return (a_left.max(b_left)) as f64;
                }
                let left_max = a_left.max(b_left) as f64;
                let right_min = a_right.min(b_right) as f64;
                return (left_max + right_min) / 2.0;
            } else if a_left > b_right {
                if i == 0 {
                    break;
                }
                hi = i - 1;
            } else {
                lo = i + 1;
            }
        }

        0.0
    }
}

fn main() {
    let a = vec![1, 3];
    let b = vec![2];
    let ans = Solution::find_median_sorted_arrays(a.clone(), b.clone());
    println!("a = {:?}, b = {:?}, median = {}", a, b, ans);
    assert!((ans - 2.0).abs() < 1e-9);

    let a = vec![1, 2];
    let b = vec![3, 4];
    let ans = Solution::find_median_sorted_arrays(a.clone(), b.clone());
    println!("a = {:?}, b = {:?}, median = {}", a, b, ans);
    assert!((ans - 2.5).abs() < 1e-9);

    let a = vec![0, 0];
    let b = vec![0, 0];
    let ans = Solution::find_median_sorted_arrays(a.clone(), b.clone());
    println!("a = {:?}, b = {:?}, median = {}", a, b, ans);
    assert!((ans - 0.0).abs() < 1e-9);

    let a = vec![];
    let b = vec![1];
    let ans = Solution::find_median_sorted_arrays(a.clone(), b.clone());
    println!("a = {:?}, b = {:?}, median = {}", a, b, ans);
    assert!((ans - 1.0).abs() < 1e-9);
}
