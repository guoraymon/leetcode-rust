use std::cmp::max;

/// 整数拆分
///
/// 来源：[LeetCode](https://leetcode-cn.com/problems/integer-break/)
///
/// ```text
/// 给定一个正整数 n，将其拆分为至少两个正整数的和，并使这些整数的乘积最大化。 返回你可以获得的最大乘积。
///
/// 示例 1:
/// 输入: 2
/// 输出: 1
/// 解释: 2 = 1 + 1, 1 × 1 = 1。
///
/// 示例 2:
/// 输入: 10
/// 输出: 36
/// 解释: 10 = 3 + 3 + 4, 3 × 3 × 4 = 36。
/// 说明: 你可以假设 n 不小于 2 且不大于 58。
/// ```
pub struct Solution {}

impl Solution {
    /// 动态规划
    /// ```text
    /// 将 i 拆分成 j 和 i−j 的和, 则有两种情况：
    /// - i−j 不再拆分成多个正整数，此时的乘积是 j×(i−j)；
    /// - i−j 继续拆分成多个正整数，此时的乘积是 j×dp[i−j]。
    ///
    /// 状态初始定义：dp[1] = 1, dp[2] = 1
    /// 状态转义方程：dp[i] = max(dp[i], max(j * (i - j), j * dp[(i - j)]));
    /// ```
    pub fn dp(n: i32) -> i32 {
        let mut dp = vec![0 as i32; (n + 1) as usize];
        dp[1] = 1;
        dp[2] = 1;

        for i in 3..n + 1 {
            for j in 1..i {
                dp[i as usize] = max(dp[i as usize], max(j * (i - j), j * dp[(i - j) as usize]));
            }
        }

        dp[n as usize]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dp() {
        assert_eq!(Solution::dp(2), 1);
        assert_eq!(Solution::dp(10), 36);
    }
}
