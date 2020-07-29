/// 最长公共前缀
///
/// 来源：[LeetCode](https://leetcode-cn.com/problems/longest-common-prefix/)
///
/// ```text
/// 编写一个函数来查找字符串数组中的最长公共前缀。
/// 如果不存在公共前缀，返回空字符串 ""。
///
/// 示例 1:
/// 输入: ["flower","flow","flight"]
/// 输出: "fl"
///
/// 示例 2:
/// 输入: ["dog","racecar","car"]
/// 输出: ""
/// 解释: 输入不存在公共前缀。
///
/// 说明:
/// 所有输入只包含小写字母 a-z 。
/// ```
///
pub struct Solution {}

impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        match strs.len() {
            0 => String::from(""),
            _ => {
                let mut prefix = strs[0].clone();
                for i in 0..strs.len() {
                    loop {
                        match strs[i].find(prefix.as_str()) {
                            Some(0) => break,
                            _ => {
                                if let None = prefix.pop() {
                                    return String::from("");
                                }
                            }
                        }
                    }
                }
                prefix
            }
        }
    }
}

#[test]
fn test() {
    assert_eq!(Solution::longest_common_prefix(vec![]), "");
    assert_eq!(
        Solution::longest_common_prefix(vec![String::from("abc")]),
        "abc"
    );
    assert_eq!(
        Solution::longest_common_prefix(vec![
            String::from("c"),
            String::from("acc"),
            String::from("ccc")
        ]),
        ""
    );
    assert_eq!(
        Solution::longest_common_prefix(vec![
            String::from("dog"),
            String::from("racecar"),
            String::from("car")
        ]),
        ""
    );
    assert_eq!(
        Solution::longest_common_prefix(vec![
            String::from("flower"),
            String::from("flow"),
            String::from("flight")
        ]),
        "fl"
    );
}
