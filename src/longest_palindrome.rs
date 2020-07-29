use std::collections::HashMap;

/// 最长回文串
///
/// 来源：[LeetCode](https://leetcode-cn.com/problems/longest-palindrome/)
///
/// ```text
/// 给定一个包含大写字母和小写字母的字符串，找到通过这些字母构造成的最长的回文串。
/// 在构造过程中，请注意区分大小写。比如 "Aa" 不能当做一个回文字符串。
///
/// 注意:
/// 假设字符串的长度不会超过 1010。
///
/// 示例 1:
///
/// 输入:
/// "abccccdd"
///
/// 输出:
/// 7
///
/// 解释:
/// 我们可以构造的最长的回文串是"dccaccd", 它的长度是 7。
/// ```
pub struct Solution {}

impl Solution {
    pub fn longest_palindrome(s: String) -> i32 {
        // 字符计次(字符仅包含大写及小写字母)
        let mut count = HashMap::new();
        for char in s.chars() {
            let c = count.entry(char).or_insert(0);
            *c += 1;
        }

        let mut ret = 0;
        for (_, value) in count.iter() {
            ret += value - value % 2
        }

        if ret != s.len() as i32 {
            ret += 1;
        }

        ret as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(Solution::longest_palindrome(String::from("abccccdd")), 7);
        assert_eq!(Solution::longest_palindrome(String::from("AAAAAA")), 6);
        assert_eq!(
            Solution::longest_palindrome(String::from(
                "zeusnilemacaronimaisanitratetartinasiaminoracamelinsuez"
            )),
            55
        );
    }
}
