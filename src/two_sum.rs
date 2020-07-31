use std::collections::HashMap;

/// 两数之和
///
/// 来源：[LeetCode](https://leetcode-cn.com/problems/two-sum/)
///
/// ```text
/// 给定一个整数数组 nums 和一个目标值 target，请你在该数组中找出和为目标值的那 两个 整数，并返回他们的数组下标。
/// 你可以假设每种输入只会对应一个答案。但是，数组中同一个元素不能使用两遍。
///
/// 示例:
/// 给定 nums = [2, 7, 11, 15], target = 9
/// 因为 nums[0] + nums[1] = 2 + 7 = 9
/// 所以返回 [0, 1]
/// ```
pub struct Solution {}

impl Solution {
    /// 暴力遍历
    /// ```text
    /// 遍历数组，并查找剩余部分是否有与 target - nums[i] 相等的值
    /// ```
    pub fn simple(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let len = nums.len();
        for i in 0..len {
            for j in (i + 1)..len {
                if nums[i] + nums[j] == target {
                    return vec![i as i32, j as i32];
                }
            }
        }

        panic!();
    }

    /// 两遍哈希表
    /// ```text
    /// 先构建一次哈希表，再遍历一遍数组，查找哈希表中与 target - nums[i] 相等的值
    /// ```
    pub fn hash(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let len = nums.len();
        let mut map: HashMap<i32, usize> = HashMap::with_capacity(len);
        for i in 0..len {
            map.insert(nums[i], i);
        }

        for i in 0..len {
            let complement = target - nums[i];
            if let Some(j) = map.get(&complement) {
                if i != *j {
                    return vec![i as i32, *j as i32];
                }
            }
        }

        panic!();
    }

    /// 一遍哈希表
    /// ```text
    /// 一次遍历，检查哈希表是否有值。没有则填充，有则直接返回结果。
    /// ```
    pub fn hash_once(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map: HashMap<i32, usize> = HashMap::with_capacity(nums.len());
        for (i, num) in nums.iter().enumerate() {
            match map.get(&(target - num)) {
                Some(j) => return vec![*j as i32, i as i32],
                None => map.insert(*num, i),
            };
        }
        panic!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple() {
        assert_eq!(Solution::simple(vec!(2, 7, 11, 15), 9), vec![0, 1]);
    }

    #[test]
    fn hash() {
        assert_eq!(Solution::hash(vec!(2, 7, 11, 15), 9), vec![0, 1]);
    }

    #[test]
    fn hash_once() {
        assert_eq!(Solution::hash_once(vec!(2, 7, 11, 15), 9), vec![0, 1]);
    }
}
