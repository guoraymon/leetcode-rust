use std::cmp::max;

pub struct Solution {}

impl Solution {
    pub fn test() {
        assert_eq!(Solution::rob(vec![]), 0);
        assert_eq!(Solution::rob(vec![1]), 1);
        assert_eq!(Solution::rob(vec![2, 1]), 2);
        assert_eq!(Solution::rob(vec![1, 2, 3, 1]), 4);
        assert_eq!(Solution::rob(vec![2, 7, 9, 3, 1]), 12);
    }

    /** 动态规划
    状态初始定义：dp[0] = nums[0], dp[1] = max(nums[0], nums[1])
    状态转义方程：dp[n] = max(nums[n] + dp[n-2], dp[n-1])
    */
    pub fn rob(nums: Vec<i32>) -> i32 {
        let len = nums.len();
        if len == 0 {
            return 0;
        }

        if len == 1 {
            return nums[0];
        }

        if len == 2 {
            return max(nums[0], nums[1]);
        }

        let mut dp = vec![0; max(2, nums.len())];
        dp[0] = nums[0];
        dp[1] = max(nums[0], nums[1]);

        for i in 2..dp.len() {
            dp[i] = max(nums[i] + dp[i - 2], dp[i - 1]);
        }
        dp[len - 1]
    }
}
