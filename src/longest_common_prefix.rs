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
