/// 拼写单词
///
/// 题目来源：[LeetCode](https://leetcode-cn.com/problems/find-words-that-can-be-formed-by-characters/)
///
/// ```text
/// 给你一份『词汇表』（字符串数组） words 和一张『字母表』（字符串） chars。
/// 假如你可以用 chars 中的『字母』（字符）拼写出 words 中的某个『单词』（字符串），那么我们就认为你掌握了这个单词。
/// 注意：每次拼写（指拼写词汇表中的一个单词）时，chars 中的每个字母都只能用一次。
/// 返回词汇表 words 中你掌握的所有单词的 长度之和。
///
/// 示例 1：
/// 输入：words = ["cat","bt","hat","tree"], chars = "atach"
/// 输出：6
/// 解释：
/// 可以形成字符串 "cat" 和 "hat"，所以答案是 3 + 3 = 6。
///
/// 示例 2：
/// 输入：words = ["hello","world","leetcode"], chars = "welldonehoneyr"
/// 输出：10
/// 解释：
/// 可以形成字符串 "hello" 和 "world"，所以答案是 5 + 5 = 10。
///
/// 提示：
/// 1 <= words.length <= 1000
/// 1 <= words[i].length, chars.length <= 100
/// 所有字符串中都仅包含小写英文字母
/// ```
pub struct Solution {}

impl Solution {
    pub fn count_characters(words: Vec<String>, chars: String) -> i32 {
        let mut res = 0;
        let chars_count = Solution::chars_count(&chars);
        'outer: for word in words {
            let words_count = Solution::chars_count(&word);

            // 对比两个单词对应字母数量
            for i in 0..26 {
                if chars_count[i] < words_count[i] {
                    continue 'outer;
                }
            }

            res += word.len();
        }

        res as i32
    }

    // 返回单词中字母出现次数的数组(单词仅包含小写字母)
    pub fn chars_count(chars: &String) -> [u8; 26] {
        let mut count = [0 as u8; 26];
        for char in chars.chars() {
            count[(char as u8 - 'a' as u8) as usize] += 1;
        }
        count
    }
}

#[test]
fn test() {
    assert_eq!(
        Solution::count_characters(
            vec![
                String::from("cat"),
                String::from("bt"),
                String::from("hat"),
                String::from("tree")
            ],
            String::from("atach")
        ),
        6
    );
    assert_eq!(
        Solution::count_characters(
            vec![
                String::from("hello"),
                String::from("world"),
                String::from("leetcode")
            ],
            String::from("welldonehoneyr")
        ),
        10
    );
}
