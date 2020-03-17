pub struct Solution {}

/// 1160. 拼写单词
/// https://leetcode-cn.com/problems/find-words-that-can-be-formed-by-characters/
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
