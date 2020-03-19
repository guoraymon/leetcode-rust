use std::collections::HashMap;

pub struct Solution {}

/// 409. 最长回文串
/// https://leetcode-cn.com/problems/longest-palindrome/
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
    extern crate test;
    use test::Bencher;

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

    #[bench]
    fn bench_add_two(b: &mut Bencher) {
        b.iter(|| Solution::longest_palindrome(String::from("AAAAAA")));
    }
}
