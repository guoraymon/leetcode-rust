use std::cmp::max;

pub struct Solution {}

impl Solution {
    /**
    动态规划
    状态定义：持股或不持股
        持股：1.前一天持股, 2.今日购入
        不持股：1.昨日不持股，2.今日卖出
    状态初始定义：dp[0] = [-prices[0], 0]
    状态转义方程：dp[n] = [max(dp[n-1][0], dp[n-1][1] -prices[n]), max(dp[n-1][1], dp[n-1][0] + prices[n])]
    */
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let len = prices.len();
        match len {
            0 => 0,
            1 => 0,
            _ => {
                let mut dp = vec![vec![0; 2]; len];
                dp[0] = vec![-prices[0], 0];

                for i in 1..len {
                    dp[i][0] = max(dp[i - 1][0], dp[i - 1][1] - prices[i]);
                    dp[i][1] = max(dp[i - 1][1], dp[i - 1][0] + prices[i]);
                }

                return dp.last().unwrap()[1];
            }
        }
    }
}

#[test]
fn test() {
    assert_eq!(Solution::max_profit(vec![]), 0);
    assert_eq!(Solution::max_profit(vec![1]), 0);
    assert_eq!(Solution::max_profit(vec![7, 6, 4, 3, 1]), 0);
    assert_eq!(Solution::max_profit(vec![1, 2, 3, 4, 5]), 4);
    assert_eq!(Solution::max_profit(vec![7, 1, 5, 3, 6, 4]), 7);
}
