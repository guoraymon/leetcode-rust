use std::collections::HashMap;

/// 有效的括号
///
/// 来源：[LeetCode](https://leetcode-cn.com/problems/valid-parentheses/)
///
/// ```text
/// 给定一个只包括 '('，')'，'{'，'}'，'['，']' 的字符串，判断字符串是否有效。
/// 有效字符串需满足：
/// 1.左括号必须用相同类型的右括号闭合。
/// 2.左括号必须以正确的顺序闭合。
/// 注意空字符串可被认为是有效字符串。
///
/// 示例 1:
/// 输入: "()"
/// 输出: true
///
/// 示例2:
/// 输入: "()[]{}"
/// 输出: true
///
/// 示例3:
/// 输入: "(]"
/// 输出: false
///
/// 示例4:
/// 输入: "([)]"
/// 输出: false
///
/// 示例5:
/// 输入: "{[]}"
/// 输出: true
/// ```
pub struct Solution {}

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack = vec![];
        let map: HashMap<char, char> = [(')', '('), (']', '['), ('}', '{')]
            .iter()
            .cloned()
            .collect();

        for char in s.chars() {
            match char {
                '(' | '[' | '{' => stack.push(char),
                ')' | ']' | '}' => {
                    if let Some(c) = stack.pop() {
                        if c != map[&char] {
                            return false;
                        }
                    } else {
                        return false;
                    }
                }
                _ => return false,
            }
        }

        stack.is_empty()
    }
}

#[test]
fn test() {
    assert_eq!(Solution::is_valid(String::from("()")), true);
    assert_eq!(Solution::is_valid(String::from("()[]{}")), true);
    assert_eq!(Solution::is_valid(String::from("(]")), false);
    assert_eq!(Solution::is_valid(String::from("([)]")), false);
    assert_eq!(Solution::is_valid(String::from("{[]}")), true);
    assert_eq!(Solution::is_valid(String::from("[")), false);
    assert_eq!(Solution::is_valid(String::from("]")), false);
}
