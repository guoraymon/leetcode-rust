/// 魔术索引
///
/// 来源：[LeetCode](https://leetcode-cn.com/problems/magic-index-lcci/)
///
/// ```text
/// 给定两个字符串形式的非负整数 num1 和num2 ，计算它们的和。
///
/// 注意：
///
/// num1 和num2 的长度都小于 5100.
/// num1 和num2 都只包含数字 0-9.
/// num1 和num2 都不包含任何前导零。
/// 你不能使用任何內建 BigInteger 库， 也不能直接将输入的字符串转换为整数形式。
/// ```
pub struct Solution {}

impl Solution {
    /// 模拟竖式加法
    pub fn add_strings(mut num1: String, mut num2: String) -> String {
        let mut res = String::new();
        let mut add = 0;

        // 遍历两数并相加
        loop {
            if num1.is_empty() && num2.is_empty() && add == 0 {
                break;
            }
            if let Some(n1) = num1.pop() {
                add += n1.to_digit(10).unwrap();
            }
            if let Some(n2) = num2.pop() {
                add += n2.to_digit(10).unwrap();
            }
            res.push_str(&(add % 10).to_string());
            add /= 10; // 进位
        }

        res.chars().rev().collect() // 反转字符串：转为 chars 后 反转
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dp() {
        assert_eq!(
            Solution::add_strings(String::from("0"), String::from("0")),
            String::from("0")
        );
        assert_eq!(
            Solution::add_strings(String::from("1"), String::from("9")),
            String::from("10")
        );
        assert_eq!(
            Solution::add_strings(String::from("12"), String::from("3")),
            String::from("15")
        );
    }
}
