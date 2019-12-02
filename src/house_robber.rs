use std::cmp::max;

pub struct Solution {}

impl Solution {
    /** 动态规划
    状态初始定义：dp[0] = nums[0], dp[1] = max(nums[0], nums[1])
    状态转义方程：dp[n] = max(nums[n] + dp[n-2], dp[n-1])
    */
    pub fn rob(nums: Vec<i32>) -> i32 {
        let len = nums.len();
        if len == 0 {
            return 0;
        }

        let mut dp = vec![0; max(2, nums.len())];
        if len >= 1 {
            dp[0] = nums[0]
        }

        if len >= 2 {
            dp[1] = max(nums[0], nums[1])
        }

        let mut res = max(dp[0], dp[1]);
        for i in 2..dp.len() {
            dp[i] = max(nums[i] + dp[i - 2], dp[i - 1]);
            res = max(res, dp[i]);
        }
        res
    }
}
