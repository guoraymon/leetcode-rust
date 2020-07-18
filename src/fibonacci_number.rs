use std::cmp::max;

pub struct Solution {}

/// 509. 斐波那契数
///
/// 斐波那契数，通常用 F(n) 表示，形成的序列称为斐波那契数列。该数列由 0 和 1 开始，后面的每一项数字都是前面两项数字的和。也就是：
///
/// ```
/// F(0) = 0,   F(1) = 1
/// F(N) = F(N - 1) + F(N - 2), 其中 N > 1.
/// 0 <= N <= 30
/// ```
///
/// 给定 N，计算 F(N)。
impl Solution {
    /// 递归
    pub fn recursion(n: i32) -> i32 {
        if n < 2 {
            n
        } else {
            Solution::recursion(n - 1) + Solution::recursion(n - 2)
        }
    }

    /// 迭代
    pub fn iter(n: i32) -> i32 {
        fn iter(n1: i32, n: i32, count: i32) -> i32 {
            if count == 0 {
                n
            } else {
                iter(n1 + n, n1, count - 1)
            }
        }

        if n < 2 {
            n
        } else {
            iter(2, 1, n - 2)
        }
    }

    /// 动态规划
    ///
    /// ```
    /// 状态初始定义：dp[0] = 0, dp[1] = 1
    /// 状态转移方程：dp[n] = dp[n-1] + dp[n-2]
    /// ```
    pub fn dp(n: i32) -> i32 {
        let n = n as usize;
        let mut dp = vec![0; max(2, n + 1)];
        dp[0] = 0;
        dp[1] = 1;

        for i in 2..n + 1 {
            dp[i] = dp[i - 1] + dp[i - 2];
        }

        dp[n]
    }

    /// 通项公式
    ///
    /// ```
    /// F(n) = (((1+√5)/2)^n - ((1-√5)/2)^n)/√5
    /// ```
    pub fn formula(n: i32) -> i32 {
        let s = 5f64.sqrt();
        (((1f64 + s) / 2f64).powi(n) / s + 0.5f64).floor() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn recursion() {
        assert_eq!(Solution::recursion(0), 0);
        assert_eq!(Solution::recursion(1), 1);
        assert_eq!(Solution::recursion(2), 1);
        assert_eq!(Solution::recursion(3), 2);
        assert_eq!(Solution::recursion(4), 3);
        assert_eq!(Solution::recursion(30), 832040);
    }

    #[test]
    fn iter() {
        assert_eq!(Solution::iter(0), 0);
        assert_eq!(Solution::iter(1), 1);
        assert_eq!(Solution::iter(2), 1);
        assert_eq!(Solution::iter(3), 2);
        assert_eq!(Solution::iter(4), 3);
        assert_eq!(Solution::iter(30), 832040);
    }

    #[test]
    fn dp() {
        assert_eq!(Solution::dp(0), 0);
        assert_eq!(Solution::dp(1), 1);
        assert_eq!(Solution::dp(2), 1);
        assert_eq!(Solution::dp(3), 2);
        assert_eq!(Solution::dp(4), 3);
        assert_eq!(Solution::dp(30), 832040);
    }

    #[test]
    fn formula() {
        assert_eq!(Solution::formula(0), 0);
        assert_eq!(Solution::formula(1), 1);
        assert_eq!(Solution::formula(2), 1);
        assert_eq!(Solution::formula(3), 2);
        assert_eq!(Solution::formula(4), 3);
        assert_eq!(Solution::formula(30), 832040);
    }
}
