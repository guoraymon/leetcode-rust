use std::cmp::min;

pub struct Solution {}

#[test]
fn test() {
    assert_eq!(Solution::minimum_total(vec![]), 0);
    assert_eq!(Solution::minimum_total(vec![vec![2]]), 2);
    assert_eq!(
        Solution::minimum_total(vec![vec![2], vec![3, 4], vec![6, 5, 7], vec![4, 1, 8, 3]]),
        11
    );
}

impl Solution {
    pub fn minimum_total(mut triangle: Vec<Vec<i32>>) -> i32 {
        match triangle.len() {
            0 => 0,
            1 => triangle[0][0],
            _ => {
                for i in (0..triangle.len()).rev() {
                    for j in (0..triangle[i].len()).rev() {
                        let t1 = triangle.get(i + 1);
                        triangle[i][j] += t1.map(|t1| min(t1[j], t1[j + 1])).unwrap_or(0);
                    }
                }
                return triangle[0][0];
            }
        }
    }
}
