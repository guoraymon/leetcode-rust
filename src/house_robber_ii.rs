use std::cmp::max;

#[test]
fn test() {
    assert_eq!(Solution::rob(vec![]), 0);
    assert_eq!(Solution::rob(vec![1]), 1);
    assert_eq!(Solution::rob(vec![2, 1]), 2);
    assert_eq!(Solution::rob(vec![2, 3, 2]), 3);
    assert_eq!(Solution::rob(vec![1, 2, 3, 1]), 4);
}

pub struct Solution {}

impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        match nums.len() {
            0 => 0,
            1 => nums[0],
            _ => max(
                Solution::rob_i(&nums[1..]),
                Solution::rob_i(&nums[..nums.len() - 1]),
            ),
        }
    }

    pub fn rob_i(nums: &[i32]) -> i32 {
        match nums.len() {
            1 => nums[0],
            2 => max(nums[0], nums[1]),
            _ => {
                let mut dp = vec![0; nums.len()];
                dp[0] = nums[0];
                dp[1] = max(nums[0], nums[1]);

                for i in 2..dp.len() {
                    dp[i] = max(nums[i] + dp[i - 2], dp[i - 1]);
                }
                *dp.last().unwrap()
            }
        }
    }
}
