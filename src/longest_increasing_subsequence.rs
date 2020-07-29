use std::cmp::max;

/// 最长上升子序列
///
/// 来源：[LeetCode](https://leetcode-cn.com/problems/longest-increasing-subsequence/)
///
/// ```text
/// 给定一个无序的整数数组，找到其中最长上升子序列的长度。
///
/// 示例:
/// 输入: [10,9,2,5,3,7,101,18]
/// 输出: 4
/// 解释: 最长的上升子序列是 [2,3,7,101]，它的长度是 4。
///
/// 说明:
/// 可能会有多种最长上升子序列的组合，你只需要输出对应的长度即可。
/// 你算法的时间复杂度应该为 O(n2) 。
/// 进阶: 你能将算法的时间复杂度降低到 O(n log n) 吗?
/// ```
pub struct Solution {}

impl Solution {
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        let len = nums.len();
        match len {
            0 => 0,
            1 => 1,
            _ => {
                let mut dp = vec![0; len];
                dp[0] = 1;
                let mut maxans = 1;
                for i in 1..len {
                    let mut maxval = 0;
                    for j in 0..i {
                        if nums[i] > nums[j] {
                            maxval = max(maxval, dp[j]);
                        }
                    }
                    dp[i] = maxval + 1;
                    maxans = max(maxans, dp[i]);
                }
                maxans
            }
        }
    }
}

#[test]
fn test() {
    assert_eq!(Solution::length_of_lis(vec![]), 0);
    assert_eq!(Solution::length_of_lis(vec![1]), 1);
    assert_eq!(Solution::length_of_lis(vec![2, 1]), 1);
    assert_eq!(Solution::length_of_lis(vec![1, 2]), 2);
    assert_eq!(Solution::length_of_lis(vec![4, 10, 4, 3, 8, 9]), 3);
    assert_eq!(Solution::length_of_lis(vec![10, 9, 2, 5, 3, 7, 101, 18]), 4);
}
