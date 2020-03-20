use std::cmp::Reverse;
use std::collections::BinaryHeap;

pub struct Solution {}

/// 面试题40. 最小的k个数
/// https://leetcode-cn.com/problems/zui-xiao-de-kge-shu-lcof/
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
    extern crate test;
    use test::Bencher;

    #[test]
    fn test() {
        assert_eq!(Solution::get_least_numbers(vec![3, 2, 1], 2), vec![1, 2]);
        assert_eq!(Solution::get_least_numbers(vec![0, 1, 2, 1], 1), vec![0]);
    }

    #[bench]
    fn bench_add_two(b: &mut Bencher) {
        b.iter(|| Solution::get_least_numbers(vec![3, 2, 1], 2));
    }
}
