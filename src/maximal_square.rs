use std::cmp::{max, min};

pub struct Solution {}

impl Solution {
    pub fn maximal_square(matrix: Vec<Vec<char>>) -> i32 {
        let row = matrix.len();
        if row == 0 {
            return 0;
        }
        let col = matrix[0].len();

        let mut dp = vec![0; col + 1];
        let mut prev = 0;
        let mut max_len = 0;
        for i in 1..=row {
            for j in 1..=col {
                let temp = dp[j];
                if matrix[i - 1][j - 1] == '1' {
                    dp[j] = min(prev, min(dp[j], dp[j - 1])) + 1;
                    max_len = max(max_len, dp[j]);
                } else {
                    dp[j] = 0;
                }
                prev = temp;
            }
        }
        (max_len * max_len) as i32
    }
}

#[test]
fn test() {
    assert_eq!(Solution::maximal_square(vec![]), 0);
    assert_eq!(Solution::maximal_square(vec![vec!['0', '1']]), 1);
    assert_eq!(
        Solution::maximal_square(vec![vec!['0', '1', '1'], vec!['0', '1', '1']]),
        4
    );
    assert_eq!(
        Solution::maximal_square(vec![
            vec!['1', '0', '1', '0', '0'],
            vec!['1', '0', '1', '1', '1'],
            vec!['1', '1', '1', '1', '1'],
            vec!['1', '0', '0', '1', '0']
        ]),
        4
    );
}
