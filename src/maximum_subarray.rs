use std::cmp::max;

/// 最大子序和
///
/// 来源：[LeetCode](https://leetcode-cn.com/problems/maximum-subarray/)
///
/// ```text
/// 给定一个整数数组 nums ，找到一个具有最大和的连续子数组（子数组最少包含一个元素），返回其最大和。
///
/// 示例:
/// 输入: [-2,1,-3,4,-1,2,1,-5,4]
/// 输出: 6
/// 解释: 连续子数组 [4,-1,2,1] 的和最大，为 6。
///
/// 进阶:
/// 如果你已经实现复杂度为 O(n) 的解法，尝试使用更为精妙的分治法求解。
/// ```
pub struct Solution {}

impl Solution {
    /**
    标准动规
    */
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let len = nums.len();
        match len {
            0 => 0,
            1 => nums[0],
            _ => {
                let mut dp = vec![0; len];
                dp[0] = nums[0];
                let mut res = dp[0];
                for i in 1..len {
                    dp[i] = max(nums[i], dp[i - 1] + nums[i]);
                    res = max(res, dp[i]);
                }
                res
            }
        }
    }
}

#[test]
fn test() {
    assert_eq!(Solution::max_sub_array(vec![]), 0);
    assert_eq!(Solution::max_sub_array(vec![1]), 1);
    assert_eq!(Solution::max_sub_array(vec![-2, -1]), -1);
    assert_eq!(
        Solution::max_sub_array(vec![-2, 1, -3, 4, -1, 2, 1, -5, 4]),
        6
    );
}
