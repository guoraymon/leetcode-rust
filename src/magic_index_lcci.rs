/// 魔术索引
///
/// 来源：[LeetCode](https://leetcode-cn.com/problems/magic-index-lcci/)
///
/// ```text
/// 魔术索引。 在数组A[0...n-1]中，有所谓的魔术索引，满足条件A[i] = i。给定一个有序整数数组，编写一种方法找出魔术索引，若有的话，在数组A中找出一个魔术索引，如果没有，则返回-1。若有多个魔术索引，返回索引值最小的一个。
///
/// 示例1:
///  输入：nums = [0, 2, 3, 4, 5]
///  输出：0
///  说明: 0下标的元素为0
///
/// 示例2:
///  输入：nums = [1, 1, 1]
///  输出：1
///
/// 提示:
/// nums长度在[1, 1000000]之间
/// ```
pub struct Solution {}

impl Solution {
    /// 遍历
    pub fn find_magic_index(nums: Vec<i32>) -> i32 {
        for i in 0..nums.len() {
            if i == nums[i] as usize {
                return i as i32;
            }
        }

        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dp() {
        assert_eq!(Solution::find_magic_index(vec!(0, 2, 3, 4, 5)), 0);
        assert_eq!(Solution::find_magic_index(vec!(1, 1, 1)), 1);
    }
}
