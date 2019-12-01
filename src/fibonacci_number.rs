use std::cmp::max;

pub struct Solution {}

impl Solution {
    /**
    动态规划
    依题意得：
    状态初始定义：dp[0] = 0, dp[1] = 1
    状态转义方程：dp[n] = dp[n-1] + dp[n-2]
    0 <= N <= 30
    */
    pub fn fib(n: i32) -> i32 {
        let n = n as usize;
        let mut dp = vec![0; max(2, n + 1)];
        dp[0] = 0;
        dp[1] = 1;

        for i in 2..n + 1 {
            dp[i] = dp[i - 1] + dp[i - 2];
        }

        dp[n]
    }
}
