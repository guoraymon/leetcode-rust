use std::cmp::max;

pub struct Solution {}

impl Solution {
    /**
    动态规划
    状态初始定义：dp[0] = [-prices[0], 0]
    状态转义方程：dp[n] = [max(dp[n-1][1], -prices[n]), dp[n-1][1] + prices[n]]
    状态定义为两种，持股或抛售。
    持股有两种情况，一种是前一天已经持股，另一种是今天购入持股
    抛售只有一种情况，即抛售前一天的持股
    */
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        if prices.len() == 0 {
            return 0;
        }

        let mut dp = vec![vec![0; 2]; prices.len()];
        dp[0] = vec![-prices[0], 0];

        let mut res = 0;
        for i in 1..prices.len() {
            dp[i][0] = max(dp[i - 1][0], -prices[i]);
            dp[i][1] = dp[i - 1][0] + prices[i];

            res = max(res, max(dp[i][0], dp[i][1]));
        }

        res
    }
}
