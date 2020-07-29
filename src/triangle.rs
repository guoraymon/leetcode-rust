use std::cmp::min;

/// 三角形最小路径和
///
/// 来源：[LeetCode](https://leetcode-cn.com/problems/triangle/)
///
/// ```text
/// 给定一个三角形，找出自顶向下的最小路径和。每一步只能移动到下一行中相邻的结点上。
/// 相邻的结点 在这里指的是 下标 与 上一层结点下标 相同或者等于 上一层结点下标 + 1 的两个结点。
///
/// 例如，给定三角形：
/// [
///      [2],
///     [3,4],
///    [6,5,7],
///   [4,1,8,3]
/// ]
/// 自顶向下的最小路径和为 11（即，2 + 3 + 5 + 1 = 11）。
///
/// 说明：
/// 如果你可以只使用 O(n) 的额外空间（n 为三角形的总行数）来解决这个问题，那么你的算法会很加分。
///
/// ```
pub struct Solution {}

#[test]
fn test() {
    assert_eq!(Solution::minimum_total(vec![]), 0);
    assert_eq!(Solution::minimum_total(vec![vec![2]]), 2);
    assert_eq!(
        Solution::minimum_total(vec![vec![2], vec![3, 4], vec![6, 5, 7], vec![4, 1, 8, 3]]),
        11
    );
}

impl Solution {
    pub fn minimum_total(mut triangle: Vec<Vec<i32>>) -> i32 {
        match triangle.len() {
            0 => 0,
            1 => triangle[0][0],
            _ => {
                for i in (0..triangle.len()).rev() {
                    for j in (0..triangle[i].len()).rev() {
                        let t1 = triangle.get(i + 1);
                        triangle[i][j] += t1.map(|t1| min(t1[j], t1[j + 1])).unwrap_or(0);
                    }
                }
                return triangle[0][0];
            }
        }
    }
}
