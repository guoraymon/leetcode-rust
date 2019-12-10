use std::cmp::max;

pub struct Solution {}

impl Solution {
    /**
    标准动规
    */
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let len = nums.len();
        match len {
            0 => 0,
            1 => nums[0],
            _ => {
                let mut dp = vec![0; len];
                dp[0] = nums[0];
                let mut res = dp[0];
                for i in 1..len {
                    dp[i] = max(nums[i], dp[i - 1] + nums[i]);
                    res = max(res, dp[i]);
                }
                res
            }
        }
    }
}

#[test]
fn test() {
    assert_eq!(Solution::max_sub_array(vec![]), 0);
    assert_eq!(Solution::max_sub_array(vec![1]), 1);
    assert_eq!(Solution::max_sub_array(vec![-2, -1]), -1);
    assert_eq!(
        Solution::max_sub_array(vec![-2, 1, -3, 4, -1, 2, 1, -5, 4]),
        6
    );
}
