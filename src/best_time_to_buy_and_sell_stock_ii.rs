use std::cmp::max;

/// 买卖股票的最佳时机 II
///
/// 来源：[LeetCode](https://leetcode-cn.com/problems/best-time-to-buy-and-sell-stock-ii)
///
/// ```text
/// 给定一个数组，它的第 i 个元素是一支给定股票第 i 天的价格。
/// 设计一个算法来计算你所能获取的最大利润。你可以尽可能地完成更多的交易（多次买卖一支股票）。
/// 注意：你不能同时参与多笔交易（你必须在再次购买前出售掉之前的股票）。
///
/// 示例 1:
/// 输入: [7,1,5,3,6,4]
/// 输出: 7
/// 解释: 在第 2 天（股票价格 = 1）的时候买入，在第 3 天（股票价格 = 5）的时候卖出, 这笔交易所能获得利润 = 5-1 = 4 。
///      随后，在第 4 天（股票价格 = 3）的时候买入，在第 5 天（股票价格 = 6）的时候卖出, 这笔交易所能获得利润 = 6-3 = 3 。
///
/// 示例 2:
/// 输入: [1,2,3,4,5]
/// 输出: 4
/// 解释: 在第 1 天（股票价格 = 1）的时候买入，在第 5 天 （股票价格 = 5）的时候卖出, 这笔交易所能获得利润 = 5-1 = 4 。
///      注意你不能在第 1 天和第 2 天接连购买股票，之后再将它们卖出。
///      因为这样属于同时参与了多笔交易，你必须在再次购买前出售掉之前的股票。
///
/// 示例 3:
/// 输入: [7,6,4,3,1]
/// 输出: 0
/// 解释: 在这种情况下, 没有交易完成, 所以最大利润为 0。
///
/// 提示：
/// 1 <= prices.length <= 3 * 10 ^ 4
/// 0 <= prices[i] <= 10 ^ 4
/// ```
pub struct Solution {}

impl Solution {
    /// 动态规划
    ///
    /// ```text
    /// 状态定义：持股或不持股
    /// 持股：1.前一天持股, 2.今日购入
    /// 不持股：1.昨日不持股，2.今日卖出
    /// 状态初始定义：dp[0] = [-prices[0], 0]
    /// 状态转义方程：dp[n] = [max(dp[n-1][0], dp[n-1][1] -prices[n]), max(dp[n-1][1], dp[n-1][0] + prices[n])]
    /// ```
    pub fn dp(prices: Vec<i32>) -> i32 {
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
    assert_eq!(Solution::dp(vec![]), 0);
    assert_eq!(Solution::dp(vec![1]), 0);
    assert_eq!(Solution::dp(vec![7, 6, 4, 3, 1]), 0);
    assert_eq!(Solution::dp(vec![1, 2, 3, 4, 5]), 4);
    assert_eq!(Solution::dp(vec![7, 1, 5, 3, 6, 4]), 7);
}
