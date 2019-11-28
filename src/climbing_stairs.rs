pub struct Solution {}

impl Solution {
    /**
    动态规划
    */
    pub fn climb_stairs(n: i32) -> i32 {
        let n = n as usize;
        let mut dp = vec![0; n + 1];
        dp[1] = 1;
        dp[2] = 2;

        for i in 3..n + 1 {
            dp[n] = dp[i - 1] + dp[i - 2];
        }

        dp[n]
    }
}
