/// 实现 strStr()
///
/// 来源：[LeetCode](https://leetcode-cn.com/problems/implement-strstr/)
///
/// 实现 `strStr()` 函数。
/// 给你两个字符串 `haystack` 和 `needle` ，请你在 `haystack` 字符串中找出 `needle` 字符串出现的第一个位置（下标从 0 开始）。如果不存在，则返回  `-1` 。
///
/// 说明：
/// 当 `needle` 是空字符串时，我们应当返回什么值呢？这是一个在面试中很好的问题。
/// 对于本题而言，当 `needle` 是空字符串时我们应当返回 0 。这与 C 语言的 `strstr()` 以及 Java 的 `indexOf()` 定义相符。
///
/// 示例 1：
/// ```text
/// 输入：haystack = "hello", needle = "ll"
/// 输出：2
/// ```
///
/// 示例 2：
/// ```text
/// 输入：haystack = "aaaaa", needle = "bba"
/// 输出：-1
/// ```
///
/// 示例 3：
/// ```text
/// 输入：haystack = "", needle = ""
/// 输出：0
/// ```
///
/// 提示：
/// - `0 <= haystack.length, needle.length <= 5 * 104`
/// - `haystack` 和 `needle` 仅由小写英文字符组成
pub struct Solution {}

impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        let haystack = haystack.into_bytes();
        let needle = needle.into_bytes();
        let n = haystack.len();
        let m = needle.len();
        if m == 0 {
            return 0;
        }

        let mut pi = vec![0; m];

        let mut j = 0;
        for i in 1..m {
            while j > 0 && needle[i] != needle[j] {
                j = pi[j - 1];
            }
            if needle[i] == needle[j] {
                j += 1;
            }
            pi[i] = j;
        }

        let mut j = 0;
        for i in 0..n {
            while j > 0 && haystack[i] != needle[j] {
                j = pi[j - 1]
            }
            if haystack[i] == needle[j] {
                j += 1;
            }
            if j == m {
                return (i - m + 1) as i32;
            }
        }
        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(
            Solution::str_str(String::from("hello"), String::from("ll")),
            2
        );
        assert_eq!(
            Solution::str_str(String::from("aaaaa"), String::from("bba")),
            -1
        );
        assert_eq!(Solution::str_str(String::from(""), String::from("")), 0);
    }
}
