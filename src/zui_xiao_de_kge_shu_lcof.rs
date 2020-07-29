use std::cmp::Reverse;
use std::collections::BinaryHeap;

pub struct Solution {}

/// 最小的k个数
///
/// 来源：[LeetCode](https://leetcode-cn.com/problems/zui-xiao-de-kge-shu-lcof/)
///
/// ```text
/// 输入整数数组 arr ，找出其中最小的 k 个数。例如，输入4、5、1、6、2、7、3、8这8个数字，则最小的4个数字是1、2、3、4。
///
/// 示例 1：
/// 输入：arr = [3,2,1], k = 2
/// 输出：[1,2] 或者 [2,1]
///
/// 示例 2：
/// 输入：arr = [0,1,2,1], k = 1
/// 输出：[0]
///
/// 限制：
/// 0 <= k <= arr.length <= 10000
/// 0 <= arr[i] <= 10000
/// ```
impl Solution {
    pub fn get_least_numbers(arr: Vec<i32>, k: i32) -> Vec<i32> {
        Solution::get_least_numbers_2(arr, k)
    }

    // 直接排序... 70 ns/iter (+/- 5)
    pub fn get_least_numbers_1(mut arr: Vec<i32>, k: i32) -> Vec<i32> {
        arr.sort();
        arr.truncate(k as usize);
        arr
    }

    // 二叉堆 328 ns/iter (+/- 13)
    pub fn get_least_numbers_2(arr: Vec<i32>, k: i32) -> Vec<i32> {
        let mut heap = BinaryHeap::new();
        for val in arr {
            heap.push(Reverse(val));
        }

        let mut res: Vec<i32> = Vec::new();
        for _ in 0..k {
            if let Some(Reverse(v)) = heap.pop() {
                res.push(v);
            }
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(Solution::get_least_numbers(vec![3, 2, 1], 2), vec![1, 2]);
        assert_eq!(Solution::get_least_numbers(vec![0, 1, 2, 1], 1), vec![0]);
    }
}
