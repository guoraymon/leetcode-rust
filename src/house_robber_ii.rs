use std::cmp::max;

/// 打家劫舍 II
///
/// 来源：[LeetCode](https://leetcode-cn.com/problems/house-robber-ii)
///
/// ```text
/// 你是一个专业的小偷，计划偷窃沿街的房屋，每间房内都藏有一定的现金。这个地方所有的房屋都围成一圈，这意味着第一个房屋和最后一个房屋是紧挨着的。同时，相邻的房屋装有相互连通的防盗系统，如果两间相邻的房屋在同一晚上被小偷闯入，系统会自动报警。
/// 给定一个代表每个房屋存放金额的非负整数数组，计算你在不触动警报装置的情况下，能够偷窃到的最高金额。
/// 示例 1:
/// 输入: [2,3,2]
/// 输出: 3
/// 解释: 你不能先偷窃 1 号房屋（金额 = 2），然后偷窃 3 号房屋（金额 = 2）, 因为他们是相邻的。
///
/// 示例 2:
/// 输入: [1,2,3,1]
/// 输出: 4
/// 解释: 你可以先偷窃 1 号房屋（金额 = 1），然后偷窃 3 号房屋（金额 = 3）。
///      偷窃到的最高金额 = 1 + 3 = 4 。
/// ```
pub struct Solution {}

impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        match nums.len() {
            0 => 0,
            1 => nums[0],
            _ => max(
                Solution::rob_i(&nums[1..]),
                Solution::rob_i(&nums[..nums.len() - 1]),
            ),
        }
    }

    pub fn rob_i(nums: &[i32]) -> i32 {
        match nums.len() {
            1 => nums[0],
            2 => max(nums[0], nums[1]),
            _ => {
                let mut dp = vec![0; nums.len()];
                dp[0] = nums[0];
                dp[1] = max(nums[0], nums[1]);

                for i in 2..dp.len() {
                    dp[i] = max(nums[i] + dp[i - 2], dp[i - 1]);
                }
                *dp.last().unwrap()
            }
        }
    }
}

#[test]
fn test() {
    assert_eq!(Solution::rob(vec![]), 0);
    assert_eq!(Solution::rob(vec![1]), 1);
    assert_eq!(Solution::rob(vec![2, 1]), 2);
    assert_eq!(Solution::rob(vec![2, 3, 2]), 3);
    assert_eq!(Solution::rob(vec![1, 2, 3, 1]), 4);
}
