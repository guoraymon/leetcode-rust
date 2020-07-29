/// 爬楼梯
///
/// 来源：[LeetCode](https://leetcode-cn.com/problems/climbing-stairs/)
///
/// ```text
/// 假设你正在爬楼梯。需要 n 阶你才能到达楼顶。
/// 每次你可以爬 1 或 2 个台阶。你有多少种不同的方法可以爬到楼顶呢？
/// 注意：给定 n 是一个正整数。
///
/// 示例 1：
/// 输入： 2
/// 输出： 2
/// 解释： 有两种方法可以爬到楼顶。
/// 1.  1 阶 + 1 阶
/// 2.  2 阶
///
/// 示例 2：
/// 输入： 3
/// 输出： 3
/// 解释： 有三种方法可以爬到楼顶。
/// 1.  1 阶 + 1 阶 + 1 阶
/// 2.  1 阶 + 2 阶
/// 3.  2 阶 + 1 阶
///```
pub struct Solution {}

impl Solution {
    /// 动态规划
    /// ```text
    /// 状态初始定义：dp[1] = 1, dp[2] = 2
    /// 状态转义方程：dp[n] = dp[i - 1] + dp[i - 2]
    /// ```
    pub fn dp(n: i32) -> i32 {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dp() {
        assert_eq!(Solution::dp(2), 2);
        assert_eq!(Solution::dp(3), 3);
    }
}
